/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetOrdersResponse : Модель для ответа API списка информации по заказам.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetOrdersResponse {
    #[serde(rename = "pager", skip_serializing_if = "Option::is_none")]
    pub pager: Option<Box<crate::models::FlippingPagerDto>>,
    /// Модель заказа.
    #[serde(rename = "orders", skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<crate::models::OrderDto>>,
}

impl GetOrdersResponse {
    /// Модель для ответа API списка информации по заказам.
    pub fn new() -> GetOrdersResponse {
        GetOrdersResponse {
            pager: None,
            orders: None,
        }
    }
}
