use anyhow::Result;
use rust_yandexmarket::models::UpdateCampaignOfferDto;
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
    let offer = UpdateCampaignOfferDto::builder()
        .offer_id("AW Carolus 75 0.8x1.5 - 2 pcs")
        .quantum(1, 1)
        .available(true)
        .vat(6)
        .build()?;
    for campaign_id in client.campaign_ids() {
        let result = client.offers_update(campaign_id, vec![offer.clone()]).await?;
        info!("Offers update result:\n{result:#?}");

    }
    Ok(())
}
