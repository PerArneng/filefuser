use std::io;
use std::future::Future;
use std::pin::Pin;
use std::path::{Path, PathBuf};
use std::collections::VecDeque;
use std::fs;

use tokio::task;
use tokio::fs as tokio_fs;
use crate::fs::async_fs::{AsyncFS, FileContent, FileEntry, FileMetadata};

/// A new "SimpleV3AsyncFS" implementation that does not depend on `glob`.
pub struct SimpleAsyncFS;

impl SimpleAsyncFS {
    /// Creates a new instance of SimpleV3AsyncFS.
    pub fn new() -> Self {
        SimpleAsyncFS
    }
}

impl AsyncFS for SimpleAsyncFS {
    fn to_absolute_path<'a>(
        &'a self,
        path: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<String, io::Error>> + Send + 'a>> {
        Box::pin(async move {
            let path_owned = path.to_string();
            // Use `spawn_blocking` so that a potentially large canonicalize operation
            // does not block the async runtime.
            let absolute = task::spawn_blocking(move || {
                let canonical = fs::canonicalize(&path_owned)?;
                let abs_str = canonical
                    .to_str()
                    .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"))?;
                Ok(abs_str.to_string())
            })
                .await
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("JoinError: {e}")))??;

            Ok(absolute)
        })
    }

    fn file_exists<'a>(
        &'a self,
        path: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<bool, io::Error>> + Send + 'a>> {
        Box::pin(async move {
            let exists = tokio_fs::try_exists(path).await?;
            Ok(exists)
        })
    }

    fn get_metadata<'a>(
        &'a self,
        path: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<FileMetadata, io::Error>> + Send + 'a>> {
        Box::pin(async move {
            let metadata = tokio_fs::metadata(path).await?;
            Ok(FileMetadata {
                size: metadata.len(),
            })
        })
    }

    fn read_file<'a>(
        &'a self,
        file_entry: &'a FileEntry,
    ) -> Pin<Box<dyn Future<Output = Result<FileContent, io::Error>> + Send + 'a>> {
        Box::pin(async move {
            let content = tokio_fs::read(&file_entry.absolute_path).await?;
            Ok(FileContent {
                entry: file_entry.clone(),
                content,
            })
        })
    }

    fn write_file<'a>(
        &'a self,
        path: &'a str,
        content: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = Result<(), io::Error>> + Send + 'a>> {
        Box::pin(async move {
            // Create parent directories if they don't exist
            if let Some(parent) = Path::new(path).parent() {
                tokio_fs::create_dir_all(parent).await?;
            }
            tokio_fs::write(path, content).await
        })
    }

    fn scan_directory<'a>(
        &'a self,
        dir: &'a str,
        patterns: &'a [String],
    ) -> Pin<Box<dyn Future<Output = Result<Vec<FileEntry>, io::Error>> + Send + 'a>> {
        Box::pin(async move {
            let mut results = Vec::new();
            let mut dirs_to_process = VecDeque::new();

            // Start from the specified directory
            dirs_to_process.push_back(PathBuf::from(dir));

            // Iterative approach using a queue
            while let Some(current_dir) = dirs_to_process.pop_front() {
                // Use spawn_blocking for reading a potentially large directory
                let entries = task::spawn_blocking({
                    let cd = current_dir.clone();
                    move || {
                        let mut paths = Vec::new();
                        for entry in fs::read_dir(cd)? {
                            let entry = entry?;
                            paths.push(entry.path());
                        }
                        Ok(paths) as Result<Vec<PathBuf>, io::Error>
                    }
                })
                    .await
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("JoinError: {e}")))??;

                for path in entries {
                    let metadata = match tokio_fs::metadata(&path).await {
                        Ok(m) => m,
                        Err(e) => {
                            return Err(io::Error::new(
                                e.kind(),
                                format!("Failed to get metadata for {}: {e}", path.display()),
                            ));
                        }
                    };

                    if metadata.is_dir() {
                        // Keep traversing into subdirectories
                        dirs_to_process.push_back(path);
                    } else if metadata.is_file() {
                        // For each pattern, check if the file path contains the pattern
                        if let Some(path_str) = path.to_str() {
                            // If no patterns provided, we include everything
                            let mut include = patterns.is_empty();
                            if !include {
                                for pat in patterns {
                                    if path_str.contains(pat) {
                                        include = true;
                                        break;
                                    }
                                }
                            }

                            if include {
                                // Get absolute path
                                let abs_path = self.to_absolute_path(path_str).await?;
                                // Retrieve metadata
                                let file_metadata = self.get_metadata(path_str).await?;

                                results.push(FileEntry {
                                    absolute_path: abs_path,
                                    metadata: file_metadata,
                                });
                            }
                        }
                    }
                }
            }

            Ok(results)
        })
    }
}
