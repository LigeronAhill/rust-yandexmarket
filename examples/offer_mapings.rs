use anyhow::Result;
use rust_yandexmarket::MarketClient;
use tracing::info;
use rust_yandexmarket::models::GetOfferMappingsRequest;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let token = std::env::var("MARKET_TOKEN").expect("MARKET_TOKEN must be set");
    let client = MarketClient::new(token)?;
    info!("Client initialized successfully\n{client:#?}");
    let business_id = 919862;
    let request = Some(GetOfferMappingsRequest::builder()
        .vendor_names(vec!["Haima".to_string()])
        .build()?);
    let offer_mappings = client.offer_mappings(business_id, None, None, request).await?;
    info!("Offer mappings: {:#?}", offer_mappings);
    Ok(())
}
