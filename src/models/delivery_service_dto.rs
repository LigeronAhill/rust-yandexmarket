/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// DeliveryServiceDto : Служба доставки.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeliveryServiceDto {
    /// Идентификатор службы доставки.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Наименование службы доставки.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl DeliveryServiceDto {
    /// Служба доставки.
    pub fn new() -> DeliveryServiceDto {
        DeliveryServiceDto {
            id: None,
            name: None,
        }
    }
}