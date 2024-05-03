use rust_yandexmarket::MarketClient;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let client = MarketClient::init().await?;
    let name = "Тамбов";
    let regions = client.geobases().search_region(name).await?;
    if let Some(first) = regions.first() {
        let region = client.geobases().get_region(first.id).await?;
        if let Some(first_region) = region.first() {
            println!("Got region with type {}", first_region.region_type);
        }
        let children = client.geobases().get_children_regions(first.id).await?;
        println!(
            "region {} has {} children regions",
            first.name,
            children.len()
        );
    }
    Ok(())
}
