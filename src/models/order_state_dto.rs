/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OrderStateDto : Информация по заказу.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrderStateDto {
    /// Идентификатор заказа.
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "status")]
    pub status: crate::models::OrderStatusType,
    #[serde(rename = "substatus", skip_serializing_if = "Option::is_none")]
    pub substatus: Option<crate::models::OrderSubstatusType>,
}

impl OrderStateDto {
    /// Информация по заказу.
    pub fn new(id: i64, status: crate::models::OrderStatusType) -> OrderStateDto {
        OrderStateDto {
            id,
            status,
            substatus: None,
        }
    }
}
