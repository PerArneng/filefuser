
use std::error::Error;
use std::process::exit;
use log::{error, info};
use crate::dirscan::{get_files};
use crate::file_data::model::FileDataExtractor;
use crate::file_data::claude::ClaudeFileDataExtractorImpl;

mod args;
mod dirscan;
mod io_utils;
mod logging;
mod file_info;
mod file_data;

async fn start() -> Result<(), Box<dyn Error>> {
    let args = args::parse_args()?;

    info!("start: output file path: {:?}", args.output_file_path);
    info!("start: file type: {:?}", args.file_type);
    info!("start: patterns: {:?}", args.patterns);

    let files = get_files(&args.search_dir, &args.patterns)?;
    info!("start: got {:?} files", files.len());

    let file_infos = file_info::get_file_infos(&files).await;
    info!("start: got {:?} file info's", files.len());

    let file_data_extractor: Box<dyn FileDataExtractor> = Box::new(ClaudeFileDataExtractorImpl::new());

    let file_data_list = match file_data_extractor.get_file_data(&files).await {
        Ok(data) => data,
        Err(e) => return Err(Box::<dyn Error>::from(e.to_string())),
    };

    info!("start: got {:?} file data's", files.len());

    // Your for loop is also problematic - file_data_list is a Vec<FileData>, not a Vec<Result<FileData, _>>
    for file_data in &file_data_list {
        info!("start: file_data: {:?}", file_data);
    }


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
