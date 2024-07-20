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
use std::fmt::Display;

/// PriceCompetitivenessType : Привлекательность цены:  * `OPTIMAL` — привлекательная. * `AVERAGE` — умеренная. * `LOW` — непривлекательная.

/// Привлекательность цены:  * `OPTIMAL` — привлекательная. * `AVERAGE` — умеренная. * `LOW` — непривлекательная.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PriceCompetitivenessType {
    #[serde(rename = "OPTIMAL")]
    Optimal,
    #[serde(rename = "AVERAGE")]
    Average,
    #[serde(rename = "LOW")]
    Low,
}

impl Display for PriceCompetitivenessType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Optimal => String::from("OPTIMAL"),
            Self::Average => String::from("AVERAGE"),
            Self::Low => String::from("LOW"),
        };
        write!(f, "{}", str)
    }
}

impl Default for PriceCompetitivenessType {
    fn default() -> PriceCompetitivenessType {
        Self::Optimal
    }
}
