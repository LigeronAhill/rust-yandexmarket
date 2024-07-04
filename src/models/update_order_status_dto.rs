/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// UpdateOrderStatusDto : Список заказов.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateOrderStatusDto {
    /// Идентификатор заказа.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::OrderStatusType>,
    #[serde(rename = "substatus", skip_serializing_if = "Option::is_none")]
    pub substatus: Option<crate::models::OrderSubstatusType>,
    #[serde(rename = "updateStatus", skip_serializing_if = "Option::is_none")]
    pub update_status: Option<crate::models::OrderUpdateStatusType>,
    /// Ошибка при изменении статуса заказа. Содержит описание ошибки и идентификатор заказа.  Возвращается, если параметр `updateStatus` принимает значение `ERROR`.
    #[serde(rename = "errorDetails", skip_serializing_if = "Option::is_none")]
    pub error_details: Option<String>,
}

impl UpdateOrderStatusDto {
    /// Список заказов.
    pub fn new() -> UpdateOrderStatusDto {
        UpdateOrderStatusDto {
            id: None,
            status: None,
            substatus: None,
            update_status: None,
            error_details: None,
        }
    }
}
