/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */
use std::fmt::Display;
use serde::{Deserialize, Serialize};

/// ApiResponseStatusType : Тип ответа.

/// Тип ответа.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ApiResponseStatusType {
    #[serde(rename = "OK")]
    Ok,
    #[serde(rename = "ERROR")]
    Error,
}

impl Display for ApiResponseStatusType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Ok => String::from("OK"),
            Self::Error => String::from("ERROR"),
        };
        write!(f, "{}", str)
    }
}

impl Default for ApiResponseStatusType {
    fn default() -> ApiResponseStatusType {
        Self::Ok
    }
}
