/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// FeedbackCommentAuthorDto : Информация об авторе ответа.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FeedbackCommentAuthorDto {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::FeedbackCommentAuthorType>,
    /// Имя автора отзыва или название магазина.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl FeedbackCommentAuthorDto {
    /// Информация об авторе ответа.
    pub fn new() -> FeedbackCommentAuthorDto {
        FeedbackCommentAuthorDto {
            r#type: None,
            name: None,
        }
    }
}