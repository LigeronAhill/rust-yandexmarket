/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OfferMappingKindType : Вид маппинга.

/// Вид маппинга.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OfferMappingKindType {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "ALL")]
    All,
}

impl ToString for OfferMappingKindType {
    fn to_string(&self) -> String {
        match self {
            Self::Active => String::from("ACTIVE"),
            Self::All => String::from("ALL"),
        }
    }
}

impl Default for OfferMappingKindType {
    fn default() -> OfferMappingKindType {
        Self::Active
    }
}
