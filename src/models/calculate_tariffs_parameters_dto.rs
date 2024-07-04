/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// CalculateTariffsParametersDto : Параметры для расчета стоимости услуг.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CalculateTariffsParametersDto {
    /// Идентификатор кампании. У пользователя, который выполняет запрос, должен быть доступ к этой кампании.  Используйте параметр `campaignId`, если уже завершили подключение магазина на Маркете. Иначе вернется пустой список.  Обязательный параметр, если не указан параметр `sellingProgram`. Совместное использование параметров приведет к ошибке.
    #[serde(rename = "campaignId", skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<i64>,
    #[serde(rename = "sellingProgram", skip_serializing_if = "Option::is_none")]
    pub selling_program: Option<crate::models::SellingProgramType>,
    #[serde(rename = "frequency", skip_serializing_if = "Option::is_none")]
    pub frequency: Option<crate::models::PaymentFrequencyType>,
}

impl CalculateTariffsParametersDto {
    /// Параметры для расчета стоимости услуг.
    pub fn new() -> CalculateTariffsParametersDto {
        CalculateTariffsParametersDto {
            campaign_id: None,
            selling_program: None,
            frequency: None,
        }
    }
}
