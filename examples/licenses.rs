use rust_yandexmarket::MarketClient;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = MarketClient::init().await?;
    let licenses = client.outlets().get_all_licenses(357750157).await?;
    println!("{licenses:#?}");
    Ok(())
}
