/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */
use serde::{Deserialize, Serialize};

/// CampaignDto : Информация о магазине.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CampaignDto {
    /// URL магазина.
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Идентификатор кампании.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Идентификатор плательщика в Яндекс Балансе.
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<i64>,
    #[serde(rename = "business", skip_serializing_if = "Option::is_none")]
    pub business: Option<Box<crate::models::BusinessDto>>,
    #[serde(rename = "placementType", skip_serializing_if = "Option::is_none")]
    pub placement_type: Option<crate::models::PlacementType>,
}

impl CampaignDto {
    /// Информация о магазине.
    pub fn new() -> CampaignDto {
        CampaignDto {
            domain: None,
            id: None,
            client_id: None,
            business: None,
            placement_type: None,
        }
    }
}
