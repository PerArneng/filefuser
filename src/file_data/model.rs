use std::error::Error;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;

#[derive(Debug, Clone)]
pub struct FileData {
    pub(crate) is_text: Option<bool>,
    pub(crate) path_to_file: PathBuf,
    pub(crate) size: Option<u64>,
    pub(crate) error: Option<String>
}

pub trait FileDataExtractor {

    /// Extracts metadata from a collection of files.
    ///
    /// For each file path provided, this function:
    /// 1. Creates a `FileData` instance containing the file's metadata
    /// 2. Determines the file size
    /// 3. Analyzes whether the file is text or binary
    ///
    /// If any operation fails for a specific file, the error is captured in that file's
    /// `FileData.error` field rather than failing the entire operation. This allows
    /// partial success when processing multiple files.
    ///
    /// # Arguments
    ///
    /// * `file_paths` - A slice of `PathBuf` objects pointing to the files to analyze
    ///
    /// # Returns
    ///
    /// A `Future` resolving to a `Result` containing a `Vec<FileData>` with metadata for each file
    fn get_file_data<'life>(
        &'life self,
        file_paths: &'life [PathBuf],
    ) -> Pin<Box<dyn Future<Output = Result<Vec<FileData>,
                Box<dyn Error + Send + Sync>>> + Send + 'life>>;

}