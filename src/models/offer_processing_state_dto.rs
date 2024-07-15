/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OfferProcessingStateDto : Информация о статусе публикации товара на Маркете.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OfferProcessingStateDto {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::OfferProcessingStatusType>,
    /// Причины, по которым товар не прошел модерацию.
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<crate::models::OfferProcessingNoteDto>>,
}

impl OfferProcessingStateDto {
    /// Информация о статусе публикации товара на Маркете.
    pub fn new() -> OfferProcessingStateDto {
        OfferProcessingStateDto {
            status: None,
            notes: None,
        }
    }
}