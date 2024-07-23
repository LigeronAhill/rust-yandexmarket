use anyhow::Result;
use rust_yandexmarket::MarketClient;
use rust_yandexmarket::models::UpdateStockDto;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let token = std::env::var("MARKET_TOKEN").expect("MARKET_TOKEN must be set");
    let client = MarketClient::new(token).await?;
    let stock_item = UpdateStockDto::new("AW Carolus 75 0.8x1.5 - 2 pcs", vec![4.6]);
    for campaign_id in client.campaign_ids() {
        client.update_stock(campaign_id, vec![stock_item.clone()]).await?;
    }
    Ok(())
}
