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

/// AddOffersToArchiveErrorDto : Товар, который не удалось поместить в архив.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AddOffersToArchiveErrorDto {
    /// Ваш SKU — идентификатор товара в вашей системе.  Разрешена любая последовательность длиной до 255 знаков.  Правила использования SKU:  * У каждого товара SKU должен быть свой.  * SKU товара нельзя менять — можно только удалить товар и добавить заново с новым SKU.  * Уже заданный SKU нельзя освободить и использовать заново для другого товара. Каждый товар должен получать новый идентификатор, до того никогда не использовавшийся в вашем каталоге.  [Что такое SKU и как его назначать](https://yandex.ru/support/marketplace/assortment/add/index.html#fields)
    #[serde(rename = "offerId")]
    pub offer_id: String,
    #[serde(rename = "error")]
    pub error: crate::models::AddOffersToArchiveErrorType,
}

impl AddOffersToArchiveErrorDto {
    /// Товар, который не удалось поместить в архив.
    pub fn new(
        offer_id: String,
        error: crate::models::AddOffersToArchiveErrorType,
    ) -> AddOffersToArchiveErrorDto {
        AddOffersToArchiveErrorDto { offer_id, error }
    }
}
