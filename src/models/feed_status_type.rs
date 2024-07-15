/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// FeedStatusType : Статус прайс-листа.  Возможные значения:    * `ERROR` — найдены ошибки.   * `NA` — прайс-лист не загружался более семи дней или на этапе загрузки произошла ошибка.   * `OK` — ошибок не найдено.

/// Статус прайс-листа.  Возможные значения:    * `ERROR` — найдены ошибки.   * `NA` — прайс-лист не загружался более семи дней или на этапе загрузки произошла ошибка.   * `OK` — ошибок не найдено.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FeedStatusType {
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "NA")]
    Na,
    #[serde(rename = "OK")]
    Ok,
}

impl ToString for FeedStatusType {
    fn to_string(&self) -> String {
        match self {
            Self::Error => String::from("ERROR"),
            Self::Na => String::from("NA"),
            Self::Ok => String::from("OK"),
        }
    }
}

impl Default for FeedStatusType {
    fn default() -> FeedStatusType {
        Self::Error
    }
}