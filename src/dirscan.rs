
use std::path::PathBuf;
use log::info;
use regex::Regex;
use crate::io_utils::{simple_patterns_to_regexps, to_io_err_with_context};

pub fn get_files(dir: &PathBuf, patterns: &[String])
    -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {

    info!("get_files: compiling patterns to regular expressions: {:?}", patterns);
    let compiled_patterns =
        simple_patterns_to_regexps(patterns)?;


    info!("get_files: searching for files in {:?}", dir);

    let found_files: Vec<PathBuf> = walk_path(dir)
        .filter(is_file)
        .filter(matches_patterns(&compiled_patterns))
        .collect();

    info!("get_files: found: {:?} files", found_files.len());

    Ok(found_files)
}

fn is_file(path: &PathBuf) -> bool {
    path.is_file()
}

fn matches_patterns<'a>(compiled_patterns: &'a [Regex]) -> impl Fn(&PathBuf) -> bool + 'a {
    move |path: &PathBuf| compiled_patterns.iter()
                .any(|re| re.is_match(path.to_str().unwrap()))
}

fn walk_path(path_buf: &PathBuf)
             -> impl Iterator<Item = PathBuf> {
    walkdir::WalkDir::new(path_buf)
        .into_iter()
        .filter_map(|entry| {
            if entry.is_err() {
                return None;
            }
            Some(entry.unwrap().path().to_path_buf())
        })
}