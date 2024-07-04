/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

use serde_derive::Deserialize;


use serde_derive::Serialize;


/// AddHiddenOffersRequest : Запрос на скрытие оферов.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AddHiddenOffersRequest {
    /// Список скрытых товаров.
    #[serde(rename = "hiddenOffers")]
    pub hidden_offers: Vec<crate::models::HiddenOfferDto>,
}

impl AddHiddenOffersRequest {
    /// Запрос на скрытие оферов.
    pub fn new(hidden_offers: Vec<crate::models::HiddenOfferDto>) -> AddHiddenOffersRequest {
        AddHiddenOffersRequest { hidden_offers }
    }
}
