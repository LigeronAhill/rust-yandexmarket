/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CalculateTariffsRequest {
    #[serde(rename = "parameters")]
    pub parameters: Box<crate::models::CalculateTariffsParametersDto>,
    /// Товары, для которых нужно рассчитать стоимость услуг.
    #[serde(rename = "offers")]
    pub offers: Vec<crate::models::CalculateTariffsOfferDto>,
}

impl CalculateTariffsRequest {
    pub fn new(
        parameters: crate::models::CalculateTariffsParametersDto,
        offers: Vec<crate::models::CalculateTariffsOfferDto>,
    ) -> CalculateTariffsRequest {
        CalculateTariffsRequest {
            parameters: Box::new(parameters),
            offers,
        }
    }
}