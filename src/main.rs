use std::{error::Error, io, ops::Deref};

use clap::{command, Parser};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::CrosstermBackend, Terminal};
use request::{Method, Request};
use ui::{run_app, App};

#[derive(Parser)]
#[command(name = "http-request")]
#[command(author = "Ilya S. <selivanov.ilya@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "Sends http request", long_about = None)]
struct Cli {
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

    match cli.ui {
        Some(true) => return run_ui(&cli),
        Some(false) | None => return run_cli(&cli).await,
    }
}

async fn run_cli(cli: &Cli) -> Result<(), Box<dyn Error>> {
    let mut request = Request::default();

    match &cli.url {
        Some(url) => request.url = url.deref().to_string(),
        None => panic!("No url provided"),
    }

    match &cli.method {
        Some(method) => request.method = Method::from_string(method.deref().to_string()),
        None => panic!("No http method provided"),
    }

    send_get(request).await
}

fn run_ui(cli: &Cli) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

async fn send_get(request: Request) -> Result<(), Box<dyn Error>> {
    let response = reqwest::get(request.url).await?;

    println!("{:#?}", response.status());

    Ok(())
}
