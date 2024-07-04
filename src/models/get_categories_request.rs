/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetCategoriesRequest : Параметры запроса категорий.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCategoriesRequest {
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<crate::models::LanguageType>,
}

impl GetCategoriesRequest {
    /// Параметры запроса категорий.
    pub fn new() -> GetCategoriesRequest {
        GetCategoriesRequest { language: None }
    }
}
