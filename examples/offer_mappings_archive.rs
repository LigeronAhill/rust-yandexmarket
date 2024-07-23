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
    let offer_ids = vec![String::from("AW Carolus 75 0.8x1.5 - 2 pcs")];
    let not_archived = client
        .offer_mappings_archive(offer_ids.clone())
        .await?;
    info!("Not archived: {:#?}", not_archived);
    let not_unarchived = client
        .offer_mappings_unarchive(offer_ids)
        .await?;
    info!("Not unarchived: {:#?}", not_unarchived);
    Ok(())
}
