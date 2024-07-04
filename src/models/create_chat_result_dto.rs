/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateChatResultDto : Информация о созданном чате.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateChatResultDto {
    /// Идентификатор чата.
    #[serde(rename = "chatId")]
    pub chat_id: i64,
}

impl CreateChatResultDto {
    /// Информация о созданном чате.
    pub fn new(chat_id: i64) -> CreateChatResultDto {
        CreateChatResultDto { chat_id }
    }
}