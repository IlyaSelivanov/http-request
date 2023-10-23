use std::error::Error;

use clap::{command, Parser};
use request::Request;
use ui::main_ui;

#[derive(Parser)]
#[command(name = "http-request")]
#[command(author = "Ilya S. <selivanov.ilya@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "Sends http request", long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    url: Option<String>,
    #[arg(short, long)]
    method: Option<String>,
    #[arg(long)]
    ui: Option<bool>,
}

mod request;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let request = Request::from_cli(&cli);

    match cli.ui {
        Some(true) => return run_ui(request.clone()).await,
        Some(false) | None => return run_cli(request.clone()).await,
    }
}

async fn run_cli(request: Request) -> Result<(), Box<dyn Error>> {
    send_get(request).await
}

async fn run_ui(request: Request) -> Result<(), Box<dyn Error>> {
    main_ui(request).await
}

async fn send_get(request: Request) -> Result<(), Box<dyn Error>> {
    let response = reqwest::get(request.url).await?;

    println!("{:#?}", response.status());

    Ok(())
}
