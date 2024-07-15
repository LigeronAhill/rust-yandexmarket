/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// QuarantineOfferDto : Товар в карантине.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct QuarantineOfferDto {
    /// Ваш SKU — идентификатор товара в вашей системе.  Разрешена любая последовательность длиной до 255 знаков.  Правила использования SKU:  * У каждого товара SKU должен быть свой.  * SKU товара нельзя менять — можно только удалить товар и добавить заново с новым SKU.  * Уже заданный SKU нельзя освободить и использовать заново для другого товара. Каждый товар должен получать новый идентификатор, до того никогда не использовавшийся в вашем каталоге.  [Что такое SKU и как его назначать](https://yandex.ru/support/marketplace/assortment/add/index.html#fields)
    #[serde(rename = "offerId", skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[serde(rename = "currentPrice", skip_serializing_if = "Option::is_none")]
    pub current_price: Option<Box<crate::models::BasePriceDto>>,
    #[serde(rename = "lastValidPrice", skip_serializing_if = "Option::is_none")]
    pub last_valid_price: Option<Box<crate::models::BasePriceDto>>,
    /// Причины попадания товара в карантин.
    #[serde(rename = "verdicts", skip_serializing_if = "Option::is_none")]
    pub verdicts: Option<Vec<crate::models::PriceQuarantineVerdictDto>>,
}

impl QuarantineOfferDto {
    /// Товар в карантине.
    pub fn new() -> QuarantineOfferDto {
        QuarantineOfferDto {
            offer_id: None,
            current_price: None,
            last_valid_price: None,
            verdicts: None,
        }
    }
}
