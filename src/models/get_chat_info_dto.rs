/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetChatInfoDto : Информация о чатах.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetChatInfoDto {
    /// Идентификатор чата.
    #[serde(rename = "chatId")]
    pub chat_id: i64,
    /// Идентификатор заказа.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    #[serde(rename = "type")]
    pub r#type: crate::models::ChatType,
    #[serde(rename = "status")]
    pub status: crate::models::ChatStatusType,
    /// Дата и время создания чата.  Формат даты: ISO 8601 со смещением относительно UTC. Например, `2017-11-21T00:00:00+03:00`.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Дата и время последнего сообщения в чате.
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl GetChatInfoDto {
    /// Информация о чатах.
    pub fn new(
        chat_id: i64,
        order_id: i64,
        r#type: crate::models::ChatType,
        status: crate::models::ChatStatusType,
        created_at: String,
        updated_at: String,
    ) -> GetChatInfoDto {
        GetChatInfoDto {
            chat_id,
            order_id,
            r#type,
            status,
            created_at,
            updated_at,
        }
    }
}
