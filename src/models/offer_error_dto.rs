/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OfferErrorDto : Сообщение об ошибке, связанной с размещением товара.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OfferErrorDto {
    /// Тип ошибки.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Пояснение.
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl OfferErrorDto {
    /// Сообщение об ошибке, связанной с размещением товара.
    pub fn new() -> OfferErrorDto {
        OfferErrorDto {
            message: None,
            comment: None,
        }
    }
}
