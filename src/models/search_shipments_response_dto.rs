/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// SearchShipmentsResponseDto : Информация об отгрузках.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SearchShipmentsResponseDto {
    /// Список с информацией об отгрузках.
    #[serde(rename = "shipments", skip_serializing_if = "Option::is_none")]
    pub shipments: Option<Vec<crate::models::ShipmentInfoDto>>,
    #[serde(rename = "paging", skip_serializing_if = "Option::is_none")]
    pub paging: Option<Box<crate::models::ForwardScrollingPagerDto>>,
}

impl SearchShipmentsResponseDto {
    /// Информация об отгрузках.
    pub fn new() -> SearchShipmentsResponseDto {
        SearchShipmentsResponseDto {
            shipments: None,
            paging: None,
        }
    }
}