/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// FeedIndexLogsIndexType : Тип обновления.  Возможные значения:  * `DIFF` — частичное обновление данных на Яндекс Маркете (например, обновление цен ранее опубликованных предложений и публикация новых). * `FAST_PRICE` — только обновление цен ранее опубликованных предложений. * `FULL` — полное обновление данных на Яндекс Маркете.

/// Тип обновления.  Возможные значения:  * `DIFF` — частичное обновление данных на Яндекс Маркете (например, обновление цен ранее опубликованных предложений и публикация новых). * `FAST_PRICE` — только обновление цен ранее опубликованных предложений. * `FULL` — полное обновление данных на Яндекс Маркете.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FeedIndexLogsIndexType {
    #[serde(rename = "DIFF")]
    Diff,
    #[serde(rename = "FAST_PRICE")]
    FastPrice,
    #[serde(rename = "FULL")]
    Full,
}

impl ToString for FeedIndexLogsIndexType {
    fn to_string(&self) -> String {
        match self {
            Self::Diff => String::from("DIFF"),
            Self::FastPrice => String::from("FAST_PRICE"),
            Self::Full => String::from("FULL"),
        }
    }
}

impl Default for FeedIndexLogsIndexType {
    fn default() -> FeedIndexLogsIndexType {
        Self::Diff
    }
}
