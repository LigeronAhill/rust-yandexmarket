/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// PriceSuggestOfferDto : Товар с ценами для продвижения.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PriceSuggestOfferDto {
    /// SKU на Маркете.
    #[serde(rename = "marketSku", skip_serializing_if = "Option::is_none")]
    pub market_sku: Option<i64>,
    /// Ваш SKU — идентификатор товара в вашей системе.  Разрешена любая последовательность длиной до 255 знаков.  Правила использования SKU:  * У каждого товара SKU должен быть свой.  * SKU товара нельзя менять — можно только удалить товар и добавить заново с новым SKU.  * Уже заданный SKU нельзя освободить и использовать заново для другого товара. Каждый товар должен получать новый идентификатор, до того никогда не использовавшийся в вашем каталоге.  [Что такое SKU и как его назначать](https://yandex.ru/support/marketplace/assortment/add/index.html#fields)
    #[serde(rename = "offerId", skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    /// Цены для продвижения.
    #[serde(rename = "priceSuggestion", skip_serializing_if = "Option::is_none")]
    pub price_suggestion: Option<Vec<crate::models::PriceSuggestDto>>,
}

impl PriceSuggestOfferDto {
    /// Товар с ценами для продвижения.
    pub fn new() -> PriceSuggestOfferDto {
        PriceSuggestOfferDto {
            market_sku: None,
            offer_id: None,
            price_suggestion: None,
        }
    }
}
