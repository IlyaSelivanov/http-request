use std::error::Error;

static URL: &str = "";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let response = reqwest::get(URL)
        .await?;

    println!("{:#?}", response.text().await?);

    Ok(())
}
