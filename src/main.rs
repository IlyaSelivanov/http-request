use std::error::Error;

use app::App;
use clap::{command, Parser};
use http_client::{HttpMethod, HttpRequest};

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

mod action;
mod app;
mod components;
mod http_client;
mod input;
mod tui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let method = match HttpMethod::from_str(cli.method.unwrap_or_default().as_str()) {
        Some(method) => method,
        None => HttpMethod::Get,
    };
    let url = cli.url.unwrap_or_default();

    let request = HttpRequest::new(method, url.as_str());

    match cli.ui {
        Some(true) => return run_ui().await,
        Some(false) | None => return run_cli(request).await,
    }
}

async fn run_cli(request: HttpRequest) -> Result<(), Box<dyn Error>> {
    let response = request.send().await;
    println!("{}", response.status_code);
    Ok(())
}

async fn run_ui() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();
    app.run().await?;
    Ok(())
}
