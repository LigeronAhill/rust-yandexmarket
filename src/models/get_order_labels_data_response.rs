/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetOrderLabelsDataResponse : Ответ с информацией для печати ярлыков.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetOrderLabelsDataResponse {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::ApiResponseStatusType>,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<crate::models::OrderLabelDto>>,
}

impl GetOrderLabelsDataResponse {
    /// Ответ с информацией для печати ярлыков.
    pub fn new() -> GetOrderLabelsDataResponse {
        GetOrderLabelsDataResponse {
            status: None,
            result: None,
        }
    }
}
