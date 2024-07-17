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

/// PlacementType : Модель, по которой работает магазин:  * `FBS` — FBS или Экспресс. * `FBY` — FBY. * `DBS` — DBS.

/// Модель, по которой работает магазин:  * `FBS` — FBS или Экспресс. * `FBY` — FBY. * `DBS` — DBS.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlacementType {
    #[serde(rename = "FBS")]
    Fbs,
    #[serde(rename = "FBY")]
    Fby,
    #[serde(rename = "DBS")]
    Dbs,
}

impl Display for PlacementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Fbs => String::from("FBS"),
            Self::Fby => String::from("FBY"),
            Self::Dbs => String::from("DBS"),
        };
        write!(f, "{}", str)
    }
}

impl Default for PlacementType {
    fn default() -> PlacementType {
        Self::Fbs
    }
}
