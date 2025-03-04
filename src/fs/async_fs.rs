use std::io;
use std::future::Future;
use std::pin::Pin;

/// Custom file metadata containing only the information needed.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FileMetadata {

    /// Size of the file in bytes.
    pub size: u64,

}

/// Represents a file entry discovered during directory scanning. It will never
/// be a directory, only a file.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FileEntry {

    /// The file path as a String.
    pub absolute_path: String,

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
    pub content: Vec<u8>,

}

/// An asynchronous trait that abstracts all required Filesystem IO operations
/// for the application, returning custom data types and using `std::io::Error`.
/// This version does not use the `async_trait` macro.
#[allow(dead_code)]
pub trait AsyncFS {


    /// Converts the provided path to an absolute path, returning it as a String.
    fn to_absolute_path<'a>(
        &'a self,
        path: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<String, io::Error>> + Send + 'a>>;

    /// Returns whether the file or directory at the specified path exists or not.
    fn file_exists<'a>(
        &'a self,
        path: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<bool, io::Error>> + Send + 'a>>;

    /// Recursively scans the specified directory and returns a list of file entries
    /// that match the provided glob patterns. It is only files that are returned
    /// and not directories. It will use get_metadata to get the metadata for each FileEntry.
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
    /// returning it wrapped in a custom `FileContent` struct that includes the FileEntry.
    fn read_file<'a>(
        &'a self,
        file_entry: &'a FileEntry,
    ) -> Pin<Box<dyn Future<Output = Result<FileContent, io::Error>> + Send + 'a>>;

    /// Asynchronously writes the provided `FileContent` to the file at the specified path.
    fn write_file<'a>(
        &'a self,
        path: &'a str,
        content: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = Result<(), io::Error>> + Send + 'a>>;
}
