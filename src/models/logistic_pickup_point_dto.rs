/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// LogisticPickupPointDto : Описание пункта вывоза для возврата.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LogisticPickupPointDto {
    /// Идентификатор пункта вывоза.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Название пункта вывоза.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<Box<crate::models::PickupAddressDto>>,
    /// Дополнительные инструкции к вывозу.
    #[serde(rename = "instruction", skip_serializing_if = "Option::is_none")]
    pub instruction: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::LogisticPointType>,
    /// Идентификатор логистического партнера, к которому относится логистическая точка.
    #[serde(rename = "logisticPartnerId", skip_serializing_if = "Option::is_none")]
    pub logistic_partner_id: Option<i64>,
}

impl LogisticPickupPointDto {
    /// Описание пункта вывоза для возврата.
    pub fn new() -> LogisticPickupPointDto {
        LogisticPickupPointDto {
            id: None,
            name: None,
            address: None,
            instruction: None,
            r#type: None,
            logistic_partner_id: None,
        }
    }
}
