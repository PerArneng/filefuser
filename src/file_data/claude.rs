use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::io::{Read, BufReader};
use std::fs::File;
use tokio::fs;
use tokio::task;
use std::path::PathBuf;

// Importing from the crate instead of redefining
use crate::file_data::model::{FileData, FileDataExtractor};

pub struct ClaudeFileDataExtractorImpl {
    // We could add configuration options here if needed
}

impl ClaudeFileDataExtractorImpl {
    /// Creates a new instance of the ClaudeFileDataExtractorImpl
    pub fn new() -> Self {
        Self {}
    }

    /// Checks if a file is likely a text file by examining its content
    async fn is_text_file(path: &PathBuf) -> Result<bool, String> {
        // Spawn a blocking task since file reading operations are blocking
        let path_clone = path.clone();
        let is_text = task::spawn_blocking(move || -> Result<bool, String> {
            let file = File::open(&path_clone)
                .map_err(|e| e.to_string())?;
            let mut reader = BufReader::new(file);
            let mut buffer = [0u8; 1024]; // Read first 1024 bytes

            let bytes_read = reader.read(&mut buffer)
                .map_err(|e| e.to_string())?;
            if bytes_read == 0 {
                return Ok(true); // Empty files are considered text files
            }

            // Heuristic: Check for NULL bytes or high proportion of non-ASCII chars
            let non_text_chars = buffer[..bytes_read]
                .iter()
                .filter(|&&b| b == 0 || b > 127)
                .count();

            // If more than 30% of the first 1024 bytes are non-text, consider it binary
            let threshold = bytes_read / 3;
            Ok(non_text_chars <= threshold)
        })
            .await
            .map_err(|e| e.to_string())?;

        is_text
    }
}

impl FileDataExtractor for ClaudeFileDataExtractorImpl {
    fn get_file_data<'life>(
        &'life self,
        file_paths: &'life [PathBuf],
    ) -> Pin<Box<dyn Future<Output = Result<Vec<FileData>, Box<dyn Error + Send + Sync>>> + Send + 'life>> {
        Box::pin(async move {
            let mut tasks = Vec::with_capacity(file_paths.len());

            // Create a task for each file
            for path in file_paths {
                let path_clone = path.clone();

                // Process each file in parallel
                let handle = tokio::spawn(async move {
                    let mut file_data = FileData {
                        is_text: None,
                        path_to_file: path_clone.clone(),
                        size: None,
                        error: None
                    };

                    // Get file metadata
                    match fs::metadata(&path_clone).await {
                        Ok(metadata) => {
                            file_data.size = Some(metadata.len());

                            // Then check if it's a text file
                            match Self::is_text_file(&path_clone).await {
                                Ok(is_text) => file_data.is_text = Some(is_text),
                                Err(e) => {
                                    file_data.error = Some(e);
                                },
                            }
                        },
                        Err(e) => {
                            // Store error as a string in the FileData struct
                            file_data.error = Some(e.to_string());
                        }
                    }

                    file_data
                });

                tasks.push(handle);
            }

            // Collect results from all tasks
            let mut results = Vec::with_capacity(file_paths.len());
            for task in tasks {
                match task.await {
                    Ok(file_data) => {
                        results.push(file_data);
                    },
                    Err(e) => {
                        return Err(Box::<dyn Error + Send + Sync>::from(e.to_string()));
                    }
                }
            }

            // Sort results to match original order if necessary
            results.sort_by(|a, b| a.path_to_file.cmp(&b.path_to_file));

            Ok(results)
        })
    }
}