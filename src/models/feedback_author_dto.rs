/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// FeedbackAuthorDto : Информация об авторе отзыва.  Если отзыв оставлен анонимно, параметр не возвращается.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FeedbackAuthorDto {
    /// Имя автора отзыва.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<Box<crate::models::RegionDto>>,
}

impl FeedbackAuthorDto {
    /// Информация об авторе отзыва.  Если отзыв оставлен анонимно, параметр не возвращается.
    pub fn new() -> FeedbackAuthorDto {
        FeedbackAuthorDto {
            name: None,
            region: None,
        }
    }
}
