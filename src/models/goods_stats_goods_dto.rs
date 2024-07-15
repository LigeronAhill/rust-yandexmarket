/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// GoodsStatsGoodsDto : Информация о товаре.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GoodsStatsGoodsDto {
    /// Ваш SKU — идентификатор товара в вашей системе.  Разрешена любая последовательность длиной до 255 знаков.  Правила использования SKU:  * У каждого товара SKU должен быть свой.  * SKU товара нельзя менять — можно только удалить товар и добавить заново с новым SKU.  * Уже заданный SKU нельзя освободить и использовать заново для другого товара. Каждый товар должен получать новый идентификатор, до того никогда не использовавшийся в вашем каталоге.  [Что такое SKU и как его назначать](https://yandex.ru/support/marketplace/assortment/add/index.html#fields)
    #[serde(rename = "shopSku", skip_serializing_if = "Option::is_none")]
    pub shop_sku: Option<String>,
    /// SKU на Маркете.
    #[serde(rename = "marketSku", skip_serializing_if = "Option::is_none")]
    pub market_sku: Option<i64>,
    /// Название товара.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Цена на товар, выставленная партнером.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f32>,
    /// Идентификатор категории товара на Маркете.
    #[serde(rename = "categoryId", skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i64>,
    /// Название категории товара на Маркете.
    #[serde(rename = "categoryName", skip_serializing_if = "Option::is_none")]
    pub category_name: Option<String>,
    #[serde(rename = "weightDimensions", skip_serializing_if = "Option::is_none")]
    pub weight_dimensions: Option<Box<crate::models::GoodsStatsWeightDimensionsDto>>,
    /// Информация о складах, на которых хранится товар.  Параметр не приходит, если товара нет ни на одном складе.
    #[serde(rename = "warehouses", skip_serializing_if = "Option::is_none")]
    pub warehouses: Option<Vec<crate::models::GoodsStatsWarehouseDto>>,
    /// Информация о тарифах, по которым нужно заплатить за услуги Маркета.  По некоторым услугам могут возвращаться несколько разных стоимостей. Например, в модели FBS стоимость услуги `SORTING` (обработка заказа) зависит от способа отгрузки и количества заказов в отгрузке. Подробнее о тарифах на услуги читайте [в Справке Маркета для продавцов](https://yandex.ru/support2/marketplace/ru/introduction/rates/models/).
    #[serde(rename = "tariffs", skip_serializing_if = "Option::is_none")]
    pub tariffs: Option<Vec<crate::models::TariffDto>>,
    /// Ссылки (URL) изображений товара в хорошем качестве.
    #[serde(rename = "pictures", skip_serializing_if = "Option::is_none")]
    pub pictures: Option<Vec<String>>,
}

impl GoodsStatsGoodsDto {
    /// Информация о товаре.
    pub fn new() -> GoodsStatsGoodsDto {
        GoodsStatsGoodsDto {
            shop_sku: None,
            market_sku: None,
            name: None,
            price: None,
            category_id: None,
            category_name: None,
            weight_dimensions: None,
            warehouses: None,
            tariffs: None,
            pictures: None,
        }
    }
}
