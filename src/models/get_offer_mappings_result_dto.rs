/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetOfferMappingsResultDto : Информация о товарах.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetOfferMappingsResultDto {
    #[serde(rename = "paging", skip_serializing_if = "Option::is_none")]
    pub paging: Option<Box<crate::models::ScrollingPagerDto>>,
    /// Информация о товарах.
    #[serde(rename = "offerMappings", skip_serializing_if = "Option::is_none")]
    pub offer_mappings: Option<Vec<crate::models::GetOfferMappingDto>>,
}

impl GetOfferMappingsResultDto {
    /// Информация о товарах.
    pub fn new() -> GetOfferMappingsResultDto {
        GetOfferMappingsResultDto {
            paging: None,
            offer_mappings: None,
        }
    }
}
