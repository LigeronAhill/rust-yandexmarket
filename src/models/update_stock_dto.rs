/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// UpdateStockDto : Информация об остатках одного товара на одном из складов.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateStockDto {
    /// Ваш SKU — идентификатор товара в вашей системе.  Разрешена любая последовательность длиной до 255 знаков.  Правила использования SKU:  * У каждого товара SKU должен быть свой.  * SKU товара нельзя менять — можно только удалить товар и добавить заново с новым SKU.  * Уже заданный SKU нельзя освободить и использовать заново для другого товара. Каждый товар должен получать новый идентификатор, до того никогда не использовавшийся в вашем каталоге.  [Что такое SKU и как его назначать](https://yandex.ru/support/marketplace/assortment/add/index.html#fields)
    #[serde(rename = "sku")]
    pub sku: String,
    /// Информация об остатках товара.
    #[serde(rename = "items")]
    pub items: Vec<crate::models::UpdateStockItemDto>,
}

impl UpdateStockDto {
    /// Информация об остатках одного товара на одном из складов.
    pub fn new(sku: String, items: Vec<crate::models::UpdateStockItemDto>) -> UpdateStockDto {
        UpdateStockDto { sku, items }
    }
}
