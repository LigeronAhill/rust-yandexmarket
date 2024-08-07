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
    for campaign_id in client.campaign_ids() {
        let offers = client.get_campaign_offers(campaign_id, None).await?;
        info!("Offers: {:#?}", offers);
    }
    Ok(())
}
