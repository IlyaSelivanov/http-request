use std::error::Error;

use clap::{command, Parser};

#[derive(Parser)]
#[command(name = "http-request")]
#[command(author = "Ilya S. <selivanov.ilya@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "Sends http request", long_about = None)]
struct Cli {
    #[arg(short, long)]
    url: String,
    #[arg(short, long)]
    method: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.method.as_str() {
        "get" => send_get(&cli.url).await.unwrap(),
        _ => {}
    }

    Ok(())
}

async fn send_get(url: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::get(url).await?;

    println!("{:#?}", response.status());

    Ok(())
}
