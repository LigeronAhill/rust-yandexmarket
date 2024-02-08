use rust_yandexmarket::{MarketClient, Result, UpdateCampaignOfferDTO};
#[tokio::main]
async fn main() -> Result<()> {
    let client = MarketClient::init().await?;
    let offers = client.sales_managment().get_offers().await?;
    dbg!(&offers);
    let acc = UpdateCampaignOfferDTO::builder()
        .offer_id("Homakoll_164_Prof_1.3")
        .available(false)
        .min_quantity(5)
        .step_quantity(1)
        .vat(6)
        .build();
    client.sales_managment().update_offers(vec![acc]).await?;
    Ok(())
}
