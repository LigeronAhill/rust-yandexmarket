/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// DeleteOffersFromArchiveRequest : Товары, которые нужно восстановить из архива.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteOffersFromArchiveRequest {
    /// Список товаров, которые нужно восстановить из архива.
    #[serde(rename = "offerIds")]
    pub offer_ids: Vec<String>,
}

impl DeleteOffersFromArchiveRequest {
    /// Товары, которые нужно восстановить из архива.
    pub fn new(offer_ids: Vec<String>) -> DeleteOffersFromArchiveRequest {
        DeleteOffersFromArchiveRequest { offer_ids }
    }
}
