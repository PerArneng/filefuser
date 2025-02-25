use std::path::PathBuf;

#[derive(Debug)]
pub struct FileInfo {
    pub(crate) is_text: bool,
    pub(crate) path_to_file: PathBuf,
}

pub fn get_file_info(file_path: &PathBuf) -> FileInfo {
    FileInfo {
        is_text: true,
        path_to_file: file_path.clone(),
    }
}