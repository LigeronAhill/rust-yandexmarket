/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// AddOffersToArchiveDto : Товары, которые не удалось поместить в архив.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AddOffersToArchiveDto {
    /// Список товаров, которые не удалось поместить в архив.
    #[serde(rename = "notArchivedOffers", skip_serializing_if = "Option::is_none")]
    pub not_archived_offers: Option<Vec<crate::models::AddOffersToArchiveErrorDto>>,
}

impl AddOffersToArchiveDto {
    /// Товары, которые не удалось поместить в архив.
    pub fn new() -> AddOffersToArchiveDto {
        AddOffersToArchiveDto {
            not_archived_offers: None,
        }
    }
}
