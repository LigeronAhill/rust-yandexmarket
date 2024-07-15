/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OfferMappingEntryDtoAllOf {
    #[serde(rename = "offer", skip_serializing_if = "Option::is_none")]
    pub offer: Option<Box<crate::models::MappingsOfferDto>>,
}

impl OfferMappingEntryDtoAllOf {
    pub fn new() -> OfferMappingEntryDtoAllOf {
        OfferMappingEntryDtoAllOf { offer: None }
    }
}
