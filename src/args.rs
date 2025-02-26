use std::path::{Path, PathBuf};
use clap::{Arg, Command};
use std::fs;
use crate::io_utils::{to_io_err_with_context};


#[derive(Debug)]
pub struct Args {
    pub(crate) output_file_path: PathBuf,
    pub(crate) file_type: String,
    pub(crate) patterns: Vec<String>,
    pub(crate) search_dir: PathBuf,
}
pub fn parse_args() -> Result<Args, Box<dyn std::error::Error>> {
    let matches = Command::new("filefuser")
        .version("0.1.0")
        .author("Per Arneng <per.arneng@scalebit.com>")
        .about("Combines text files into a single text document using mime multipart")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Sets the output file path")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("type")
                .short('t')
                .long("type")
                .value_name("TYPE")
                .help("Sets the type (default: eml)")
                .num_args(1)
                .default_value("eml"),
        )
        .arg(
            Arg::new("patterns")
                .short('p')
                .long("patterns")
                .value_name("PATTERNS")
                .help("Comma separated list of glob patterns to match files to process")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("dir")
                .short('d')
                .long("dir")
                .value_name("DIR")
                .help("Sets the search directory (default: current directory)")
                .num_args(1)
                .default_value("."),
        )
        .get_matches();


    let full_file_path: PathBuf;
    let file = matches.get_one::<String>("file").unwrap();
    let file_path = Path::new(file).to_path_buf();
    if let Some(parent) = file_path.parent() {

        if !parent.is_dir() {
            return Err(format!("parent directory '{}' is not a directory", parent.display()).into());
        }
        if !parent.exists() {
            return Err(format!("parent directory '{}' does not exist", parent.display()).into());
        }

        let full_parent_path = fs::canonicalize(parent)
            .map_err(to_io_err_with_context("making output file parent path absolute".to_string()))?;

        full_file_path = full_parent_path.join(file_path.file_name().unwrap());

    } else {
        return Err("output file has no parent directory".into());
    }

    let file_type = matches.get_one::<String>("type").unwrap().clone();
    let patterns = matches.get_one::<String>("patterns").unwrap().clone();

    let search_dir = matches.get_one::<String>("dir").unwrap().clone();

    let pattern_vec: Vec<String> = patterns
            .split(',')
            .map(|s| s.to_string())
            .map(|s| s.trim().to_string())
            .collect();

    let search_dir_path = Path::new(&search_dir).to_path_buf();
    if !search_dir_path.exists() {
        return Err(format!("directory '{}' does not exist", search_dir).into());
    }
    let full_search_dir_path = fs::canonicalize(&search_dir_path)
        .map_err(to_io_err_with_context("error making search dir absolute".to_string()))?;

    Ok(Args {
        output_file_path: full_file_path,
        file_type,
        patterns: pattern_vec,
        search_dir: full_search_dir_path,
    })
}