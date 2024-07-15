/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateChatRequest : Заказ, для которого нужно создать чат.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateChatRequest {
    /// Идентификатор заказа на Маркете.
    #[serde(rename = "orderId")]
    pub order_id: i64,
}

impl CreateChatRequest {
    /// Заказ, для которого нужно создать чат.
    pub fn new(order_id: i64) -> CreateChatRequest {
        CreateChatRequest { order_id }
    }
}