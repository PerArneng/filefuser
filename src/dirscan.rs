
use std::path::PathBuf;
use log::info;
use crate::io_utils::{simple_patterns_to_regexps, to_io_err_with_context};

pub fn get_files(dir: &PathBuf, patterns: &[String])
    -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {

    info!("get_files: searching for files in {:?}", dir);


    let compiled_patterns =
        simple_patterns_to_regexps(patterns)?;

    let mut files = Vec::new();
    Ok(files)
}

fn scan_dir(dir: &PathBuf, patterns: &[String]) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            files.append(&mut scan_dir(&path, patterns));
        } else {
            files.push(path);
        }
    }
    files
}