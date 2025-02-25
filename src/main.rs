
use std::error::Error;
use std::process::exit;
use log::{error, info};
use crate::dirscan::{get_files};

mod args;
mod dirscan;
mod io_utils;
mod logging;
mod file_info;

async fn start() -> Result<(), Box<dyn Error>> {
    let args = args::parse_args()?;

    info!("start: output file path: {:?}", args.output_file_path);
    info!("start: file type: {:?}", args.file_type);
    info!("start: patterns: {:?}", args.patterns);

    let files = get_files(&args.search_dir, &args.patterns)?;
    info!("start: got {:?} files", files.len());

    let file_infos = file_info::get_file_infos(&files).await;
    info!("start: got {:?} file info's", files.len());

    let error_file_infos: Vec<_> = file_infos.iter()
        .filter(|res| res.is_err())
        .map(|res| res.as_ref().unwrap_err().to_string())
        .collect();
    if !error_file_infos.is_empty() {
        error!("start: error getting file info's: {:?}", error_file_infos);
        exit(1);
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
