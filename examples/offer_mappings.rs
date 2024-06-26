use rust_yandexmarket::{
    MarketClient, OfferMappingRequest, UpdateBusinessOfferPriceDTO, UpdateOfferMappingDTO,
};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let client = MarketClient::init().await?;
    let offers = client.offer_mappings().get_all_offers().await?;
    dbg!(offers);
    let req = OfferMappingRequest::builder()
        .vendor_name("Homakoll")
        .build();
    let filtered_offers = client
        .offer_mappings()
        .get_all_offers_with_filter(req)
        .await?;
    dbg!(filtered_offers);
    let update = UpdateOfferMappingDTO::builder()
        .offer_id("Homakoll_164_Prof_1.3")
        .name("Клей Homakoll 164 Prof 1.3 кг")
        .category("Клей")
        .picture("https://main-cdn.sbermegamarket.ru/big2/hlr-system/335/279/913/730/125/5/600004169210b0.jpeg")
        .vendor("Homakoll")
        .description("Для коммерческого (гомогенного и гетерогенного) линолеума. Ковролина. ПВХ плитки. К любым основаниям. Морозостойкий.")
        .manufacturer_country("Россия")
        .weight_dimensions(20.0, 20.0, 20.0, 1.3)
        .build()?;
    let _ = client.offer_mappings().update_offers(vec![update]).await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    let prices = vec![UpdateBusinessOfferPriceDTO::new(
        "Homakoll_164_Prof_1.3",
        1074.0,
        None,
    )];
    let _ = client.offer_mappings().update_offers_prices(prices).await?;
    let not_archived = client
        .offer_mappings()
        .archive_offers(vec!["Homakoll_164_Prof_1.3".to_string()])
        .await?;
    dbg!(not_archived);
    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    let not_unarchived = client
        .offer_mappings()
        .unarchive_offers(vec!["Homakoll_164_Prof_1.3".to_string()])
        .await?;
    dbg!(not_unarchived);
    let to_delete = vec!["Homakoll_164_Prof_1.3".to_string()];
    let not_deleted = client.offer_mappings().delete_offers(to_delete).await?;
    dbg!(not_deleted);
    Ok(())
}
