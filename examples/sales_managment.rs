use rust_yandexmarket::{MarketClient, Result, StockDTO, UpdateCampaignOfferDTO};

#[tokio::main]
async fn main() -> Result<()> {
    let client = MarketClient::init().await?;
    let offer_id = "Homakoll_164_Prof_1.3";
    let acc = UpdateCampaignOfferDTO::builder()
        .offer_id(offer_id)
        .available(true)
        .min_quantity(5)
        .step_quantity(1)
        .vat(6)
        .build();
    client.sales_managment().update_offers(vec![acc]).await?;
    let stock = vec![StockDTO::builder()
        .sku(offer_id)
        .warehouse_id(78079)
        .count(6)
        .count(3)
        .build()];
    client.sales_managment().stock_update(stock).await?;
    let stock = client.sales_managment().retrieve_stock().await?;
    dbg!(stock);
    let filtered_stock = client
        .sales_managment()
        .retrieve_stock_with_ids(vec![offer_id.to_string()])
        .await?;
    dbg!(filtered_stock);
    Ok(())
}
