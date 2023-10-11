use std::error::Error;

use clap::{Parser, command, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Get {
        #[arg(short, long)]
        url: String
    },
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Get { url }) => {
            send_get(url).await?
        }
        None => {}
    }
    
    Ok(())
}

async fn send_get(url: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::get(url)
        .await?;

    println!("{:#?}", response.status());

    Ok(())
}