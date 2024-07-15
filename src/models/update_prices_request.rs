/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// UpdatePricesRequest : Запрос на установку цен на товары.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdatePricesRequest {
    /// Список товаров.
    #[serde(rename = "offers")]
    pub offers: Vec<crate::models::OfferPriceDto>,
}

impl UpdatePricesRequest {
    /// Запрос на установку цен на товары.
    pub fn new(offers: Vec<crate::models::OfferPriceDto>) -> UpdatePricesRequest {
        UpdatePricesRequest { offers }
    }
}