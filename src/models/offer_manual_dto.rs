/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */
use serde::{Deserialize, Serialize};

/// OfferManualDto : Инструкция по использованию товара.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OfferManualDto {
    /// Ссылка на инструкцию.
    #[serde(rename = "url")]
    pub url: String,
    /// Название инструкции, которое будет отображаться на карточке товара.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl OfferManualDto {
    /// Инструкция по использованию товара.
    pub fn new(url: String) -> OfferManualDto {
        OfferManualDto { url, title: None }
    }
}
