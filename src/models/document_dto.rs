/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// DocumentDto : Информация о документе.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DocumentDto {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::OrderDocumentStatusType>,
    /// Номер документа.
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    /// Дата создания документа.
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

impl DocumentDto {
    /// Информация о документе.
    pub fn new() -> DocumentDto {
        DocumentDto {
            status: None,
            number: None,
            date: None,
        }
    }
}
