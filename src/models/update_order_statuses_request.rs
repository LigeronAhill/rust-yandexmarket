/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// UpdateOrderStatusesRequest : Список заказов.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateOrderStatusesRequest {
    /// Список заказов.
    #[serde(rename = "orders")]
    pub orders: Vec<crate::models::OrderStateDto>,
}

impl UpdateOrderStatusesRequest {
    /// Список заказов.
    pub fn new(orders: Vec<crate::models::OrderStateDto>) -> UpdateOrderStatusesRequest {
        UpdateOrderStatusesRequest { orders }
    }
}