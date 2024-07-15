/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCategoriesMaxSaleQuantumResponse {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::ApiResponseStatusType>,
    /// Категории и лимит на установку кванта и минимального количества товаров.
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<crate::models::MaxSaleQuantumDto>>,
    /// Ошибки, которые появились из-за переданных категорий.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::CategoryErrorDto>>,
}

impl GetCategoriesMaxSaleQuantumResponse {
    pub fn new() -> GetCategoriesMaxSaleQuantumResponse {
        GetCategoriesMaxSaleQuantumResponse {
            status: None,
            results: None,
            errors: None,
        }
    }
}