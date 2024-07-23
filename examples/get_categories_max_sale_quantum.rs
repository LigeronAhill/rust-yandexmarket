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
    let category_id = 91636;
    let category_max_sale_quantum = client
        .get_categories_max_sale_quantum(vec![category_id])
        .await?;
    info!("Category max sale quantum:\n{category_max_sale_quantum:#?}");
    Ok(())
}
