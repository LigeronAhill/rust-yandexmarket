/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */
use serde::{Deserialize, Serialize};

/// AddOffersToArchiveRequest : Товары, которые нужно поместить в архив.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AddOffersToArchiveRequest {
    /// Список товаров, которые нужно поместить в архив.
    #[serde(rename = "offerIds")]
    pub offer_ids: Vec<String>,
}

impl AddOffersToArchiveRequest {
    /// Товары, которые нужно поместить в архив.
    pub fn new(offer_ids: Vec<String>) -> AddOffersToArchiveRequest {
        AddOffersToArchiveRequest { offer_ids }
    }
}
