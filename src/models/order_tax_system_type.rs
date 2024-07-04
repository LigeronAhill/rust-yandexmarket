/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OrderTaxSystemType : Система налогообложения (СНО) магазина на момент оформления заказа:  * `ECHN` — единый сельскохозяйственный налог (ЕСХН).  * `ENVD` — единый налог на вмененный доход (ЕНВД).  * `OSN` — общая система налогообложения (ОСН).  * `PSN` — патентная система налогообложения (ПСН).  * `USN` — упрощенная система налогообложения (УСН).  * `USN_MINUS_COST` — упрощенная система налогообложения, доходы, уменьшенные на величину расходов (УСН «Доходы минус расходы»).  * `NPD` — налог на профессиональный доход (НПД).  * `UNKNOWN_VALUE` — неизвестное значение. Используется только совместно с параметром `payment-method=YANDEX`.

/// Система налогообложения (СНО) магазина на момент оформления заказа:  * `ECHN` — единый сельскохозяйственный налог (ЕСХН).  * `ENVD` — единый налог на вмененный доход (ЕНВД).  * `OSN` — общая система налогообложения (ОСН).  * `PSN` — патентная система налогообложения (ПСН).  * `USN` — упрощенная система налогообложения (УСН).  * `USN_MINUS_COST` — упрощенная система налогообложения, доходы, уменьшенные на величину расходов (УСН «Доходы минус расходы»).  * `NPD` — налог на профессиональный доход (НПД).  * `UNKNOWN_VALUE` — неизвестное значение. Используется только совместно с параметром `payment-method=YANDEX`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrderTaxSystemType {
    #[serde(rename = "OSN")]
    Osn,
    #[serde(rename = "USN")]
    Usn,
    #[serde(rename = "USN_MINUS_COST")]
    UsnMinusCost,
    #[serde(rename = "ENVD")]
    Envd,
    #[serde(rename = "ECHN")]
    Echn,
    #[serde(rename = "PSN")]
    Psn,
    #[serde(rename = "NPD")]
    Npd,
    #[serde(rename = "UNKNOWN_VALUE")]
    UnknownValue,
}

impl ToString for OrderTaxSystemType {
    fn to_string(&self) -> String {
        match self {
            Self::Osn => String::from("OSN"),
            Self::Usn => String::from("USN"),
            Self::UsnMinusCost => String::from("USN_MINUS_COST"),
            Self::Envd => String::from("ENVD"),
            Self::Echn => String::from("ECHN"),
            Self::Psn => String::from("PSN"),
            Self::Npd => String::from("NPD"),
            Self::UnknownValue => String::from("UNKNOWN_VALUE"),
        }
    }
}

impl Default for OrderTaxSystemType {
    fn default() -> OrderTaxSystemType {
        Self::Osn
    }
}
