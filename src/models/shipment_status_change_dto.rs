/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// ShipmentStatusChangeDto : Статус отгрузки.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipmentStatusChangeDto {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::ShipmentStatusType>,
    /// Описание статуса отгрузки.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Время последнего изменения статуса отгрузки.
    #[serde(rename = "updateTime", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl ShipmentStatusChangeDto {
    /// Статус отгрузки.
    pub fn new() -> ShipmentStatusChangeDto {
        ShipmentStatusChangeDto {
            status: None,
            description: None,
            update_time: None,
        }
    }
}
