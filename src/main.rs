
use std::error::Error;
use std::process::exit;
use log::{error, info, warn};
use crate::dirscan::{get_files};
use crate::file_data::core::{FileData, FileDataExtractor, only_errors, only_binaries, only_text_files};
use crate::file_data::claude::ClaudeFileDataExtractorImpl;

mod args;
mod dirscan;
mod io_utils;
mod logging;
mod file_data;




async fn start() -> Result<(), Box<dyn Error>> {
    let args = args::parse_args()?;

    info!("start: output file path: {:?}", args.output_file_path);
    info!("start: file type: {:?}", args.file_type);
    info!("start: patterns: {:?}", args.patterns);

    let files = get_files(&args.search_dir, &args.patterns)?;
    info!("start: got {:?} files", files.len());

    let file_data_extractor: Box<dyn FileDataExtractor> =
        Box::new(ClaudeFileDataExtractorImpl::new());

    let file_data_list = match file_data_extractor.get_file_data(&files).await {
        Ok(data) => data,
        Err(e) => return Err(Box::<dyn Error>::from(e.to_string())),
    };

    info!("start: got {:?} file data's", file_data_list.len());

    // extract all the error lists from the file_data_list
    let errors: Vec<FileData> = only_errors(&file_data_list);
    for error in &errors {
        error!("start: error: {:?}", error);
    }

    if !errors.is_empty() {

        exit(1);
    }
    // from this point we know the list is only successful results

    let binary_files: Vec<FileData> = only_binaries(&file_data_list);
    if !binary_files.is_empty() {
        warn!("start: found {:?} binary files", binary_files.len());
        for binary_file in &binary_files {
            warn!("start: ignoring binary: {:?}", binary_file);
        }
    }

    let text_files: Vec<FileData> = only_text_files(&file_data_list);
    info!("start: found {:?} text files", text_files.len());

    Ok(())
}

#[tokio::main]
async fn main() {

    logging::init();
    info!("main: filefuser");

    if let Err(e) = start().await {
        eprintln!("Error: {}", e);
    }
}
