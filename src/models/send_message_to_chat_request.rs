/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// SendMessageToChatRequest : В какой чат нужно отправить сообщение и текст сообщения.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SendMessageToChatRequest {
    /// Текст сообщения. Максимальная длина — 4096 символа.
    #[serde(rename = "message")]
    pub message: String,
}

impl SendMessageToChatRequest {
    /// В какой чат нужно отправить сообщение и текст сообщения.
    pub fn new(message: String) -> SendMessageToChatRequest {
        SendMessageToChatRequest { message }
    }
}
