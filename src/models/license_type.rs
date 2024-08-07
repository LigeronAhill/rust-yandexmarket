/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// LicenseType : Тип лицензии:  * `ALCOHOL` — лицензия на розничную продажу алкогольной продукции.

/// Тип лицензии:  * `ALCOHOL` — лицензия на розничную продажу алкогольной продукции.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LicenseType {
    #[serde(rename = "ALCOHOL")]
    Alcohol,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl ToString for LicenseType {
    fn to_string(&self) -> String {
        match self {
            Self::Alcohol => String::from("ALCOHOL"),
            Self::Unknown => String::from("UNKNOWN"),
        }
    }
}

impl Default for LicenseType {
    fn default() -> LicenseType {
        Self::Alcohol
    }
}
