/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OrderStatusChangeDto : Заказ.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrderStatusChangeDto {
    #[serde(rename = "status")]
    pub status: crate::models::OrderStatusType,
    #[serde(rename = "substatus", skip_serializing_if = "Option::is_none")]
    pub substatus: Option<crate::models::OrderSubstatusType>,
    #[serde(rename = "delivery", skip_serializing_if = "Option::is_none")]
    pub delivery: Option<Box<crate::models::OrderStatusChangeDeliveryDto>>,
}

impl OrderStatusChangeDto {
    /// Заказ.
    pub fn new(status: crate::models::OrderStatusType) -> OrderStatusChangeDto {
        OrderStatusChangeDto {
            status,
            substatus: None,
            delivery: None,
        }
    }
}
