use std::io::Error;
use std::path::PathBuf;
use tokio::fs;
use tokio::io;

use crate::io_utils::is_text_file;

#[derive(Debug)]
pub struct FileInfo {
    pub(crate) is_text: bool,
    pub(crate) path_to_file: PathBuf,
    pub(crate) size: u64,
}

pub async fn get_file_infos(file_paths: &[PathBuf])
        -> Vec<Result<FileInfo, Box<dyn std::error::Error>>> {

    let mut file_infos = Vec::new();
    for file_path in file_paths {
        file_infos.push(get_file_info(file_path).await);
    }
    file_infos
}


pub async fn get_file_info(file_path: &PathBuf) -> Result<FileInfo, Box<dyn std::error::Error>> {

    let is_text_file = is_text_file(file_path).await?;
    let metadata = fs::metadata(file_path).await?;

    Ok(FileInfo {
        is_text: is_text_file,
        path_to_file: file_path.clone(),
        size: metadata.len(),
    })
}