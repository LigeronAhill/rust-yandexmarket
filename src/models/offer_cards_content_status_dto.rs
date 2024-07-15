/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OfferCardsContentStatusDto : Список товаров с информацией о состоянии карточек.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OfferCardsContentStatusDto {
    /// Страница списка товаров с информацией о состоянии карточек.
    #[serde(rename = "offerCards", skip_serializing_if = "Option::is_none")]
    pub offer_cards: Option<Vec<crate::models::OfferCardDto>>,
    #[serde(rename = "paging", skip_serializing_if = "Option::is_none")]
    pub paging: Option<Box<crate::models::ForwardScrollingPagerDto>>,
}

impl OfferCardsContentStatusDto {
    /// Список товаров с информацией о состоянии карточек.
    pub fn new() -> OfferCardsContentStatusDto {
        OfferCardsContentStatusDto {
            offer_cards: None,
            paging: None,
        }
    }
}
