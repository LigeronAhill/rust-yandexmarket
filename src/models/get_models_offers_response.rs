/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetModelsOffersResponse : Ответ на запрос списка предложений для моделей.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetModelsOffersResponse {
    /// Список моделей товаров.
    #[serde(rename = "models", skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<crate::models::EnrichedModelDto>>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::models::CurrencyType>,
    /// Идентификатор региона, для которого выводится информация о предложениях модели (доставляемых в этот регион).  Информацию о регионе по идентификатору можно получить с помощью запроса [GET regions/{regionId}](../../reference/regions/searchRegionsById.md).
    #[serde(rename = "regionId", skip_serializing_if = "Option::is_none")]
    pub region_id: Option<i64>,
}

impl GetModelsOffersResponse {
    /// Ответ на запрос списка предложений для моделей.
    pub fn new() -> GetModelsOffersResponse {
        GetModelsOffersResponse {
            models: None,
            currency: None,
            region_id: None,
        }
    }
}
