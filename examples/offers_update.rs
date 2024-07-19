use anyhow::Result;
use rust_yandexmarket::MarketClient;
use tracing::info;
use rust_yandexmarket::models::UpdateCampaignOfferDto;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let token = std::env::var("MARKET_TOKEN").expect("MARKET_TOKEN must be set");
    let client = MarketClient::new(token)?;
    info!("Client initialized successfully\n{client:#?}");
    let campaign_id = 112289474;
    let offer = UpdateCampaignOfferDto::builder()
        .offer_id("AW Carolus 75 0.8x1.5 - 2 pcs")
        .quantum(1,1)
        .available(true)
        .vat(6)
        .build()?;
    let result = client.offers_update(campaign_id, vec![offer]).await?;
    info!("Offers update result:\n{result:#?}");
    Ok(())
}
