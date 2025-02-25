
use std::error::Error;
use log::info;
use crate::dirscan::{get_files};

mod args;
mod dirscan;
mod io_utils;
mod logging;

async fn start() -> Result<(), Box<dyn Error>> {
    let args = args::parse_args()?;

    info!("start: output file path: {:?}", args.output_file_path);
    info!("start: file type: {:?}", args.file_type);
    info!("start: patterns: {:?}", args.patterns);

    let files = get_files(&args.search_dir, &args.patterns);
    println!("{:?}", files);

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
