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

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateOfferMappingsRequest {
    /// Перечень товаров, которые нужно добавить или обновить.
    #[serde(rename = "offerMappings")]
    pub offer_mappings: Vec<crate::models::UpdateOfferMappingDto>,
}

impl UpdateOfferMappingsRequest {
    pub fn new(
        offer_mappings: Vec<crate::models::UpdateOfferMappingDto>,
    ) -> UpdateOfferMappingsRequest {
        UpdateOfferMappingsRequest { offer_mappings }
    }
}
