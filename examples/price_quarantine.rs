use anyhow::Result;
use rust_yandexmarket::MarketClient;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let token = std::env::var("MARKET_TOKEN").expect("MARKET_TOKEN must be set");
    let client = MarketClient::new(token).await?;
    let quarantine = client.price_quarantine(None).await?;
    info!("{quarantine:#?}");
    let offer_ids = quarantine
        .into_iter()
        .flat_map(|q| q.offer_id)
        .collect::<Vec<_>>();
    client.price_quarantine_confirm(offer_ids).await?;
    Ok(())
}
