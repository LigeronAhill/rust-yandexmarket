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

/// OfferSellingProgramStatusType : Информация о доступности или недоступности.  * `FINE` — доступно. * `REJECT` — недоступно.

/// Информация о доступности или недоступности.  * `FINE` — доступно. * `REJECT` — недоступно.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OfferSellingProgramStatusType {
    #[serde(rename = "FINE")]
    Fine,
    #[serde(rename = "REJECT")]
    Reject,
}

impl Display for OfferSellingProgramStatusType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Fine => String::from("FINE"),
            Self::Reject => String::from("REJECT"),
        };
        write!(f, "{}", str)
    }
}

impl Default for OfferSellingProgramStatusType {
    fn default() -> OfferSellingProgramStatusType {
        Self::Fine
    }
}
