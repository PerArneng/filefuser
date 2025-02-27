use std::error::Error;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use crate::file_data::core::FileData;

pub trait Archiver {

    /// Creates an archive file from a list of PathBuf that contains information
    /// about the files to be archived. The archive can be of different formats
    /// but its important that the contents and info about the files are captured
    /// in the archive.
    fn archive<'life>(
        &'life self,
        file_path: &'life PathBuf,
        file_paths: &'life [PathBuf],
    ) -> Pin<Box<dyn Future<Output = Result<(),
            Box<dyn Error + Send + Sync>>> + Send + 'life>>;

}