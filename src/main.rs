use std::error::Error;

mod args;


async fn start() -> Result<(), Box<dyn Error>> {
    let args = args::parse_args()?;
    println!("{:?}", args);
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = start().await {
        eprintln!("Error: {}", e);
    }
}
