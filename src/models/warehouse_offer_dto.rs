/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// WarehouseOfferDto : Информация об остатках товара.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WarehouseOfferDto {
    /// Ваш SKU — идентификатор товара в вашей системе.  Разрешена любая последовательность длиной до 255 знаков.  Правила использования SKU:  * У каждого товара SKU должен быть свой.  * SKU товара нельзя менять — можно только удалить товар и добавить заново с новым SKU.  * Уже заданный SKU нельзя освободить и использовать заново для другого товара. Каждый товар должен получать новый идентификатор, до того никогда не использовавшийся в вашем каталоге.  [Что такое SKU и как его назначать](https://yandex.ru/support/marketplace/assortment/add/index.html#fields)
    #[serde(rename = "offerId")]
    pub offer_id: String,
    #[serde(rename = "turnoverSummary", skip_serializing_if = "Option::is_none")]
    pub turnover_summary: Option<Box<crate::models::TurnoverDto>>,
    /// Информация об остатках.
    #[serde(rename = "stocks", skip_serializing_if = "Option::is_none")]
    pub stocks: Option<Vec<crate::models::WarehouseStockDto>>,
    /// Дата и время последнего обновления информации об остатках.  Формат даты и времени: ISO 8601 со смещением относительно UTC. Например, `2023-11-21T00:42:42+03:00`.
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl WarehouseOfferDto {
    /// Информация об остатках товара.
    pub fn new(offer_id: String) -> WarehouseOfferDto {
        WarehouseOfferDto {
            offer_id,
            turnover_summary: None,
            stocks: None,
            updated_at: None,
        }
    }
}
