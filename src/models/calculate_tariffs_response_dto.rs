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

/// CalculateTariffsResponseDto : Расчет стоимости услуг.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CalculateTariffsResponseDto {
    /// Стоимость услуг.
    #[serde(rename = "offers")]
    pub offers: Vec<crate::models::CalculateTariffsOfferInfoDto>,
}

impl CalculateTariffsResponseDto {
    /// Расчет стоимости услуг.
    pub fn new(
        offers: Vec<crate::models::CalculateTariffsOfferInfoDto>,
    ) -> CalculateTariffsResponseDto {
        CalculateTariffsResponseDto { offers }
    }
}
