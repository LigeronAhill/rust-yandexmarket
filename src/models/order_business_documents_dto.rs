/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OrderBusinessDocumentsDto : Информация о документах.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrderBusinessDocumentsDto {
    #[serde(rename = "upd", skip_serializing_if = "Option::is_none")]
    pub upd: Option<Box<crate::models::DocumentDto>>,
    #[serde(rename = "ukd", skip_serializing_if = "Option::is_none")]
    pub ukd: Option<Box<crate::models::DocumentDto>>,
    #[serde(rename = "torgTwelve", skip_serializing_if = "Option::is_none")]
    pub torg_twelve: Option<Box<crate::models::DocumentDto>>,
    #[serde(rename = "sf", skip_serializing_if = "Option::is_none")]
    pub sf: Option<Box<crate::models::DocumentDto>>,
    #[serde(rename = "ksf", skip_serializing_if = "Option::is_none")]
    pub ksf: Option<Box<crate::models::DocumentDto>>,
}

impl OrderBusinessDocumentsDto {
    /// Информация о документах.
    pub fn new() -> OrderBusinessDocumentsDto {
        OrderBusinessDocumentsDto {
            upd: None,
            ukd: None,
            torg_twelve: None,
            sf: None,
            ksf: None,
        }
    }
}
