use anyhow::Result;
use rust_yandexmarket::MarketClient;
use rust_yandexmarket::models::UpdateBusinessOfferPriceDto;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let token = std::env::var("MARKET_TOKEN").expect("MARKET_TOKEN must be set");
    let client = MarketClient::new(token)?;
    let business_id = 919862;
    let offer_id = "AW Mambo 99 50x50";
    let body = vec![UpdateBusinessOfferPriceDto::new(offer_id, 26925.0, Some(31690.0))];
    client.offer_prices_updates(business_id, body).await?;
    Ok(())
}
