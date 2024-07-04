/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OrderBuyerDto : Информация о покупателе.  Параметры `id`, `lastName`, `firstName` и `middleName` возвращаются, только если вы работаете по модели DBS.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrderBuyerDto {
    /// Идентификатор покупателя.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Фамилия покупателя.
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Имя покупателя.
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Отчество покупателя.
    #[serde(rename = "middleName", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::OrderBuyerType>,
}

impl OrderBuyerDto {
    /// Информация о покупателе.  Параметры `id`, `lastName`, `firstName` и `middleName` возвращаются, только если вы работаете по модели DBS.
    pub fn new() -> OrderBuyerDto {
        OrderBuyerDto {
            id: None,
            last_name: None,
            first_name: None,
            middle_name: None,
            r#type: None,
        }
    }
}