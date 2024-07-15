/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// CalculateTariffsOfferInfoDto : Стоимость услуг.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CalculateTariffsOfferInfoDto {
    #[serde(rename = "offer")]
    pub offer: Box<crate::models::CalculateTariffsOfferDto>,
    /// Список услуг и их стоимость.  По некоторым услугам могут возвращаться несколько разных стоимостей. Например, в модели FBS стоимость услуги `SORTING` (обработка заказа) зависит от способа отгрузки и количества заказов в отгрузке. Подробнее о тарифах на услуги читайте [в Справке Маркета для продавцов](https://yandex.ru/support2/marketplace/ru/introduction/rates/models/).
    #[serde(rename = "tariffs")]
    pub tariffs: Vec<crate::models::CalculatedTariffDto>,
}

impl CalculateTariffsOfferInfoDto {
    /// Стоимость услуг.
    pub fn new(
        offer: crate::models::CalculateTariffsOfferDto,
        tariffs: Vec<crate::models::CalculatedTariffDto>,
    ) -> CalculateTariffsOfferInfoDto {
        CalculateTariffsOfferInfoDto {
            offer: Box::new(offer),
            tariffs,
        }
    }
}