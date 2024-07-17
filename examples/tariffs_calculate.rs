use anyhow::Result;
use rust_yandexmarket::MarketClient;
use tracing::info;
use rust_yandexmarket::models::{CalculateTariffsOfferDto, SellingProgramType};

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let token = std::env::var("MARKET_TOKEN").expect("MARKET_TOKEN must be set");
    let client = MarketClient::new(token)?;
    info!("Client initialized successfully\n{client:#?}");
    let offers = vec![CalculateTariffsOfferDto::new(
        6119048,
        21990.0,
        0.8,
        0.4,
        0.4,
        6.72,
        Some(1),
    )];
    let tariffs = client
        .tariffs_calculate(None, Some(SellingProgramType::Dbs), None, offers)
        .await?;
    info!("Tariffs: {:#?}", tariffs);
    Ok(())
}
