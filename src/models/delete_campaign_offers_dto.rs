/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// DeleteCampaignOffersDto : Список товаров, которые не удалось удалить, потому что они хранятся на складе Маркета.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteCampaignOffersDto {
    /// Список SKU.
    #[serde(rename = "notDeletedOfferIds", skip_serializing_if = "Option::is_none")]
    pub not_deleted_offer_ids: Option<Vec<String>>,
}

impl DeleteCampaignOffersDto {
    /// Список товаров, которые не удалось удалить, потому что они хранятся на складе Маркета.
    pub fn new() -> DeleteCampaignOffersDto {
        DeleteCampaignOffersDto {
            not_deleted_offer_ids: None,
        }
    }
}
