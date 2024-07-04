/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// EacVerificationResultDto : Результат выполнения запроса.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EacVerificationResultDto {
    #[serde(rename = "verificationResult", skip_serializing_if = "Option::is_none")]
    pub verification_result: Option<crate::models::EacVerificationStatusType>,
    /// Количество оставшихся попыток проверки кода.  Возвращается, если магазин отправил некорректный код.  Когда все попытки будут исчерпаны, код обновится.
    #[serde(rename = "attemptsLeft", skip_serializing_if = "Option::is_none")]
    pub attempts_left: Option<i32>,
}

impl EacVerificationResultDto {
    /// Результат выполнения запроса.
    pub fn new() -> EacVerificationResultDto {
        EacVerificationResultDto {
            verification_result: None,
            attempts_left: None,
        }
    }
}
