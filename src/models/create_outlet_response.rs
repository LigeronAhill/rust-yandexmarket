/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateOutletResponse : Ответ на запрос о создании точки продаж.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateOutletResponse {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::ApiResponseStatusType>,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<crate::models::OutletResponseDto>>,
}

impl CreateOutletResponse {
    /// Ответ на запрос о создании точки продаж.
    pub fn new() -> CreateOutletResponse {
        CreateOutletResponse {
            status: None,
            result: None,
        }
    }
}