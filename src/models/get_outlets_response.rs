/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetOutletsResponse : Ответ на запрос информации о точках продаж.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetOutletsResponse {
    /// Информация о точках продаж.
    #[serde(rename = "outlets", skip_serializing_if = "Option::is_none")]
    pub outlets: Option<Vec<crate::models::FullOutletDto>>,
    #[serde(rename = "paging", skip_serializing_if = "Option::is_none")]
    pub paging: Option<Box<crate::models::ScrollingPagerDto>>,
    #[serde(rename = "pager", skip_serializing_if = "Option::is_none")]
    pub pager: Option<Box<crate::models::FlippingPagerDto>>,
}

impl GetOutletsResponse {
    /// Ответ на запрос информации о точках продаж.
    pub fn new() -> GetOutletsResponse {
        GetOutletsResponse {
            outlets: None,
            paging: None,
            pager: None,
        }
    }
}
