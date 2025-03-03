use std::path::{Path, PathBuf};
use clap::{Arg, Command};
use std::{env, fs};
use crate::io_utils;
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
        .version(env!("CARGO_PKG_VERSION"))
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




    let file_arg = matches.get_one::<String>("file").unwrap();
    let mut file_path = Path::new(file_arg).to_path_buf();

    // if the given arg is just a filename then make
    // sure it gets prefixed with current dir.
    if io_utils::is_just_filename(&file_path) {
        let current_dir = env::current_dir()?;
        file_path = current_dir.join(file_path);
    }


    let full_parent_path = fs::canonicalize(file_path.parent().unwrap())?;

    if !full_parent_path.exists() {
        return Err(format!("parent directory '{}' for output file does not exist", full_parent_path.display()).into());
    }

    if !full_parent_path.is_dir() {
        return Err(format!("parent directory '{}' for output is not a directory", full_parent_path.display()).into());
    }


    let full_file_path = full_parent_path.join(file_path.file_name().unwrap());

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