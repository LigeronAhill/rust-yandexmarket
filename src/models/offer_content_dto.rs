/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OfferContentDto : Товар с указанными характеристиками.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OfferContentDto {
    /// Ваш SKU — идентификатор товара в вашей системе.  Разрешена любая последовательность длиной до 255 знаков.  Правила использования SKU:  * У каждого товара SKU должен быть свой.  * SKU товара нельзя менять — можно только удалить товар и добавить заново с новым SKU.  * Уже заданный SKU нельзя освободить и использовать заново для другого товара. Каждый товар должен получать новый идентификатор, до того никогда не использовавшийся в вашем каталоге.  [Что такое SKU и как его назначать](https://yandex.ru/support/marketplace/assortment/add/index.html#fields)
    #[serde(rename = "offerId")]
    pub offer_id: String,
    /// Идентификатор категории на Маркете. Чтобы узнать идентификатор категории, к которой относится товар, воспользуйтесь запросом [POST categories/tree](../../reference/categories/getCategoriesTree.md).
    #[serde(rename = "categoryId")]
    pub category_id: i32,
    /// Список характеристик с их значениями.  С `parameterValues` обязательно передавайте `marketCategoryId` — идентификатор категории на Маркете, к которой относятся указанные характеристики товара.  Всегда обновляется целиком.  Максимальное количество характеристик — 300.
    #[serde(rename = "parameterValues")]
    pub parameter_values: Vec<crate::models::ParameterValueDto>,
}

impl OfferContentDto {
    /// Товар с указанными характеристиками.
    pub fn new(
        offer_id: String,
        category_id: i32,
        parameter_values: Vec<crate::models::ParameterValueDto>,
    ) -> OfferContentDto {
        OfferContentDto {
            offer_id,
            category_id,
            parameter_values,
        }
    }
}
