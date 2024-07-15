/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// FeedContentErrorDto : Информация об ошибке в содержимом прайс-листа. Выводится, если параметр `content status=ERROR`.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FeedContentErrorDto {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::FeedContentErrorType>,
}

impl FeedContentErrorDto {
    /// Информация об ошибке в содержимом прайс-листа. Выводится, если параметр `content status=ERROR`.
    pub fn new() -> FeedContentErrorDto {
        FeedContentErrorDto { r#type: None }
    }
}