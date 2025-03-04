use std::io;
use std::future::Future;
use std::pin::Pin;

/// Custom file metadata containing only the information needed.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FileMetadata {
    /// Size of the file in bytes.
    pub size: u64,
    /// Indicates if the path represents a directory.
    pub is_directory: bool,
    // Additional fields can be added as needed.
}

/// Represents a file entry discovered during directory scanning.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FileEntry {
    /// The file path as a String.
    pub path: String,
    /// Custom metadata for the file.
    pub metadata: FileMetadata,
}

/// Represents the content of a file along with its associated file entry.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FileContent {
    /// The file entry containing path and metadata.
    pub entry: FileEntry,
    /// The textual content of the file.
    pub content: String,
}

/// An asynchronous trait that abstracts all required Filesystem IO operations
/// for the application, returning custom data types and using `std::io::Error`.
/// This version does not use the `async_trait` macro.
#[allow(dead_code)]
pub trait AsyncFS {
    /// Recursively scans the specified directory and returns a list of file entries
    /// that match the provided glob patterns.
    fn scan_directory<'a>(
        &'a self,
        dir: &'a str,
        patterns: &'a [String],
    ) -> Pin<Box<dyn Future<Output = Result<Vec<FileEntry>, io::Error>> + Send + 'a>>;

    /// Retrieves custom metadata for the file at the given path.
    fn get_metadata<'a>(
        &'a self,
        path: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<FileMetadata, io::Error>> + Send + 'a>>;

    /// Asynchronously reads the entire content of the file at the given path,
    /// returning it wrapped in a custom `FileContent` struct that includes the file entry.
    fn read_file<'a>(
        &'a self,
        path: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<FileContent, io::Error>> + Send + 'a>>;

    /// Asynchronously writes the provided `FileContent` to the file at the specified path.
    fn write_file<'a>(
        &'a self,
        path: &'a str,
        content: &'a FileContent,
    ) -> Pin<Box<dyn Future<Output = Result<(), io::Error>> + Send + 'a>>;
}
