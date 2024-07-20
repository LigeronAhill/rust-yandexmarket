use anyhow::Result;
use rust_yandexmarket::models::GetOfferMappingsRequest;
use rust_yandexmarket::MarketClient;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let token = std::env::var("MARKET_TOKEN").expect("MARKET_TOKEN must be set");
    let client = MarketClient::new(token)?;
    let business_id = 919862;
    let req = GetOfferMappingsRequest::builder()
        .vendor_names(vec!["Haima".to_string()])
        .build()?;
    let offer_ids = client
        .offer_mappings(business_id, Some(req))
        .await?
        .into_iter()
        .flat_map(|o| o.offer.map(|f| f.offer_id))
        .collect::<Vec<_>>();
    let not_deleted_offers = client.delete_offers(business_id, offer_ids).await?;
    info!("Not deleted offers: {not_deleted_offers:#?}");
    Ok(())
}
