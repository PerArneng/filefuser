use std::io::Error;
use std::path::PathBuf;
use log::info;
use tokio::fs;
use tokio::io;
use tokio::task::JoinError;

use crate::io_utils::is_text_file;

#[derive(Debug)]
pub struct FileInfo {
    pub(crate) is_text: bool,
    pub(crate) path_to_file: PathBuf,
    pub(crate) size: u64,
}


pub async fn get_file_infos(
    file_paths: &[PathBuf],
) -> Vec<Result<FileInfo, Box<dyn std::error::Error + Send + Sync>>> {

    info!("get_file_infos: getting file info for {} files", file_paths.len());

    // Spawn a task for each file path.
    let mut handles = Vec::with_capacity(file_paths.len());
    for file_path in file_paths {
        let file_path = file_path.clone(); // Clone to move into the async task.
        handles.push(tokio::spawn(async move {
            get_file_info(&file_path).await
        }));
    }

    // Await each join handle and collect the results.
    let mut results = Vec::with_capacity(handles.len());
    for handle in handles {
        // Await the join handle. If the task ran successfully, use its result.
        // Otherwise, convert the JoinError into a boxed error.
        match handle.await {
            Ok(res) => results.push(res),
            Err(e) => results.push(Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>)),
        }
    }

    results
}

pub async fn get_file_info(file_path: &PathBuf) -> Result<FileInfo, Box<dyn std::error::Error + Send + Sync>> {

    let is_text_file = is_text_file(file_path).await.map_err(|e| -> Box<dyn std::error::Error + Send + Sync> {
        Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    })?;
    let metadata = fs::metadata(file_path).await?;

    Ok(FileInfo {
        is_text: is_text_file,
        path_to_file: file_path.clone(),
        size: metadata.len(),
    })
}