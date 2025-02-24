use clap::{Arg, Command};

#[derive(Debug)]
pub struct Args {
    file: String,
    file_type: String,
    patterns: String,
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
        .get_matches();

    let file = matches.get_one::<String>("file").unwrap().clone();
    let file_type = matches.get_one::<String>("type").unwrap().clone();
    let patterns = matches.get_one::<String>("patterns").unwrap().clone();

    Ok(Args {
        file,
        file_type,
        patterns,
    })
}