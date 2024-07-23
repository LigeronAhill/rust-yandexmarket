use anyhow::Result;
use rust_yandexmarket::{MarketClient, SearchByName};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let token = std::env::var("MARKET_TOKEN").expect("MARKET_TOKEN must be set");
    let client = MarketClient::new(token).await?;
    let categories_tree = client.get_categories_tree().await?;
    // info!("Categories tree: {:#?}", categories_tree);
    let carpet_category = categories_tree
        .result
        .and_then(|c| c.children)
        .unwrap_or_default()
        .search_by_name("ковры");
    info!("Carpet category: {carpet_category:#?}");
    Ok(())
}
