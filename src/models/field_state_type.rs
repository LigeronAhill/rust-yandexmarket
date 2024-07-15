/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// FieldStateType : Фильтр по заполненности или незаполненности поля:  * `SPECIFIED` — вывести товары, у которых поле заполнено. * `EMPTY` — вывести товары, у которых поле не заполнено.

/// Фильтр по заполненности или незаполненности поля:  * `SPECIFIED` — вывести товары, у которых поле заполнено. * `EMPTY` — вывести товары, у которых поле не заполнено.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FieldStateType {
    #[serde(rename = "SPECIFIED")]
    Specified,
    #[serde(rename = "EMPTY")]
    Empty,
}

impl ToString for FieldStateType {
    fn to_string(&self) -> String {
        match self {
            Self::Specified => String::from("SPECIFIED"),
            Self::Empty => String::from("EMPTY"),
        }
    }
}

impl Default for FieldStateType {
    fn default() -> FieldStateType {
        Self::Specified
    }
}
