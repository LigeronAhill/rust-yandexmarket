/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetChatHistoryRequest : Историю какого чата нужно получить — и начиная с какого сообщения.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetChatHistoryRequest {
    /// Идентификатор сообщения, начиная с которого нужно получить все последующие сообщения.
    #[serde(rename = "messageIdFrom", skip_serializing_if = "Option::is_none")]
    pub message_id_from: Option<i64>,
}

impl GetChatHistoryRequest {
    /// Историю какого чата нужно получить — и начиная с какого сообщения.
    pub fn new() -> GetChatHistoryRequest {
        GetChatHistoryRequest {
            message_id_from: None,
        }
    }
}
