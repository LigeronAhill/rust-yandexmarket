/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OutletVisibilityType : Состояние точки продаж.  Возможные значения:  * `HIDDEN` — точка продаж выключена. * `VISIBLE` — точка продаж включена.

/// Состояние точки продаж.  Возможные значения:  * `HIDDEN` — точка продаж выключена. * `VISIBLE` — точка продаж включена.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OutletVisibilityType {
    #[serde(rename = "HIDDEN")]
    Hidden,
    #[serde(rename = "VISIBLE")]
    Visible,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl ToString for OutletVisibilityType {
    fn to_string(&self) -> String {
        match self {
            Self::Hidden => String::from("HIDDEN"),
            Self::Visible => String::from("VISIBLE"),
            Self::Unknown => String::from("UNKNOWN"),
        }
    }
}

impl Default for OutletVisibilityType {
    fn default() -> OutletVisibilityType {
        Self::Hidden
    }
}
