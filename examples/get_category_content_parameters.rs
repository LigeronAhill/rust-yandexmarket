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
    let client = MarketClient::new(token)?;
    info!("Client initialized successfully\n{client:#?}");
    let category_id = 91636;
    let category_content_parameters = client.get_category_content_parameters(category_id).await?;
    info!("Category content parameters: {:#?}", category_content_parameters);
    Ok(())
}
