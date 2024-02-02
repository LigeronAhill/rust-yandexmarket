use rust_yandexmarket::{MarketClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let client = MarketClient::init().await?;
    let offers = client.offer_mappings().get_all_offers().await?;
    println!("{}", offers.len());
    Ok(())
}
