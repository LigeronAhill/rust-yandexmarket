use rust_yandexmarket::{MarketClient, StockDTO, UpdateCampaignOfferDTO};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = MarketClient::init().await?;
    let offer_id = "Homakoll_164_Prof_1.3";
    let acc = UpdateCampaignOfferDTO::builder()
        .offer_id(offer_id)
        .available(false)
        .min_quantity(5)
        .step_quantity(1)
        .vat(6)
        .build();
    client.sales_management().update_offers(vec![acc]).await?;
    let stock = vec![StockDTO::builder()
        .sku(offer_id)
        .warehouse_id(78079)
        .count(6)
        .count(3)
        .build()];
    client.sales_management().stock_update(stock).await?;
    let stock = client.sales_management().retrieve_stock().await?;
    dbg!(stock);
    let filtered_stock = client
        .sales_management()
        .retrieve_stock_with_ids(vec![offer_id.to_string()])
        .await?;
    dbg!(filtered_stock);
    Ok(())
}
