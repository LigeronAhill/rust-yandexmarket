use rust_yandexmarket::{MarketClient, OfferCardRequest, OfferContentDTO, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = MarketClient::init().await?;
    let market_category_id = 15317135;
    let cards = client.offer_cards().get_all_offer_cards().await?;
    dbg!(&cards);
    let req = OfferCardRequest::builder()
        .category_id(market_category_id)
        .build();
    let filtered = client
        .offer_cards()
        .get_all_offer_cards_with_filter(req)
        .await?;
    dbg!(&filtered);
    let chars = client
        .offer_cards()
        .get_category_characteristics(market_category_id)
        .await?;
    dbg!(&chars);
    let card = OfferContentDTO::builder()
        .category_id(market_category_id)
        .offer_id("Homakoll_164_Prof_1.3")
        .parameter_value(15366467, Some(29602430), "ведро")
        .build();
    let _ = client.offer_cards().update_offer_cards(vec![card]).await?;
    Ok(())
}
