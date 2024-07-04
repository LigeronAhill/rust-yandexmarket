/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// CategoryErrorDto : Текст ошибки.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CategoryErrorDto {
    /// Идентификатор категории.
    #[serde(rename = "categoryId", skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::CategoryErrorType>,
}

impl CategoryErrorDto {
    /// Текст ошибки.
    pub fn new() -> CategoryErrorDto {
        CategoryErrorDto {
            category_id: None,
            r#type: None,
        }
    }
}
