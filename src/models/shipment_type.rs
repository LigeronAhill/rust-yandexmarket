/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// ShipmentType : Способ отгрузки заказов:  * `IMPORT` — вы самостоятельно привозите заказы в выбранный сортировочный центр или пункт приема заказов. * `WITHDRAW` — вы отгружаете заказы со своего склада курьерам Яндекс Маркета.

/// Способ отгрузки заказов:  * `IMPORT` — вы самостоятельно привозите заказы в выбранный сортировочный центр или пункт приема заказов. * `WITHDRAW` — вы отгружаете заказы со своего склада курьерам Яндекс Маркета.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ShipmentType {
    #[serde(rename = "IMPORT")]
    Import,
    #[serde(rename = "WITHDRAW")]
    Withdraw,
}

impl ToString for ShipmentType {
    fn to_string(&self) -> String {
        match self {
            Self::Import => String::from("IMPORT"),
            Self::Withdraw => String::from("WITHDRAW"),
        }
    }
}

impl Default for ShipmentType {
    fn default() -> ShipmentType {
        Self::Import
    }
}
