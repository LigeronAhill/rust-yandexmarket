/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetModelsRequest : Запрос информации о моделях.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetModelsRequest {
    /// Список моделей.
    #[serde(rename = "models", skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<i64>>,
}

impl GetModelsRequest {
    /// Запрос информации о моделях.
    pub fn new() -> GetModelsRequest {
        GetModelsRequest { models: None }
    }
}
