use rust_yandexmarket::{MarketClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = MarketClient::init().await?;
    let cards = client.offer_cards().get_all_offer_cards().await?;
    dbg!(cards);
    Ok(())
}
