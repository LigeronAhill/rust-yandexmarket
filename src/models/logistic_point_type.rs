/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// LogisticPointType : Тип логистической точки:    * `WAREHOUSE` — склад.   * `PICKUP_POINT` — обычная точка выдачи заказов (ПВЗ).   * `PICKUP_TERMINAL` — постамат.   * `PICKUP_POST_OFFICE` — отделение почтовой связи (ОПС).   * `PICKUP_MIXED` — торговый зал и пункт выдачи заказов.   * `PICKUP_RETAIL` — торговый зал.

/// Тип логистической точки:    * `WAREHOUSE` — склад.   * `PICKUP_POINT` — обычная точка выдачи заказов (ПВЗ).   * `PICKUP_TERMINAL` — постамат.   * `PICKUP_POST_OFFICE` — отделение почтовой связи (ОПС).   * `PICKUP_MIXED` — торговый зал и пункт выдачи заказов.   * `PICKUP_RETAIL` — торговый зал.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogisticPointType {
    #[serde(rename = "WAREHOUSE")]
    Warehouse,
    #[serde(rename = "PICKUP_POINT")]
    PickupPoint,
    #[serde(rename = "PICKUP_TERMINAL")]
    PickupTerminal,
    #[serde(rename = "PICKUP_POST_OFFICE")]
    PickupPostOffice,
    #[serde(rename = "PICKUP_MIXED")]
    PickupMixed,
    #[serde(rename = "PICKUP_RETAIL")]
    PickupRetail,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl ToString for LogisticPointType {
    fn to_string(&self) -> String {
        match self {
            Self::Warehouse => String::from("WAREHOUSE"),
            Self::PickupPoint => String::from("PICKUP_POINT"),
            Self::PickupTerminal => String::from("PICKUP_TERMINAL"),
            Self::PickupPostOffice => String::from("PICKUP_POST_OFFICE"),
            Self::PickupMixed => String::from("PICKUP_MIXED"),
            Self::PickupRetail => String::from("PICKUP_RETAIL"),
            Self::Unknown => String::from("UNKNOWN"),
        }
    }
}

impl Default for LogisticPointType {
    fn default() -> LogisticPointType {
        Self::Warehouse
    }
}