/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetOutletResponse : Ответ на запрос информации о точке продаж.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetOutletResponse {
    #[serde(rename = "outlet", skip_serializing_if = "Option::is_none")]
    pub outlet: Option<Box<crate::models::FullOutletDto>>,
}

impl GetOutletResponse {
    /// Ответ на запрос информации о точке продаж.
    pub fn new() -> GetOutletResponse {
        GetOutletResponse { outlet: None }
    }
}
