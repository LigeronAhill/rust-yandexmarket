/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// ModelOfferDto : Информация о предложении.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ModelOfferDto {
    /// Скидка на предложение в процентах.
    #[serde(rename = "discount", skip_serializing_if = "Option::is_none")]
    pub discount: Option<i32>,
    /// Наименование предложения.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Позиция предложения в выдаче Маркета на карточке модели.
    #[serde(rename = "pos", skip_serializing_if = "Option::is_none")]
    pub pos: Option<i32>,
    /// Цена предложения без скидки магазина.
    #[serde(rename = "preDiscountPrice", skip_serializing_if = "Option::is_none")]
    pub pre_discount_price: Option<f32>,
    /// Цена предложения без скидки, которую получает покупатель при оплате через Yandex Pay.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f32>,
    /// Идентификатор региона предложения (регион, откуда доставляется товар).  Сначала показываются предложения, доставляемые из региона, указанного в запросе в параметре `regionId`. Предложения, доставляемые из других регионов, показываются после них.
    #[serde(rename = "regionId", skip_serializing_if = "Option::is_none")]
    pub region_id: Option<i64>,
    /// Стоимость доставки товара в регион:  * `0` — доставка осуществляется бесплатно. * `-1` — магазин не осуществляет доставку этого товара (самовывоз).  Если стоимость доставки неизвестна, параметр не выводится.
    #[serde(rename = "shippingCost", skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<f32>,
    /// Название магазина (в том виде, в котором отображается на Маркете).
    #[serde(rename = "shopName", skip_serializing_if = "Option::is_none")]
    pub shop_name: Option<String>,
    /// Рейтинг магазина.  Возможные значения: * `-1` — у магазинов, недавно появившихся на Маркете, рейтинг появляется не сразу. До момента появления рейтинга для таких магазинов возвращается значение `-1`. * `1`. * `2`. * `3`. * `4`. * `5`.
    #[serde(rename = "shopRating", skip_serializing_if = "Option::is_none")]
    pub shop_rating: Option<i32>,
    /// {% note warning \"\" %}  Этот параметр устарел. Не используйте его.  {% endnote %}
    #[serde(rename = "inStock", skip_serializing_if = "Option::is_none")]
    pub in_stock: Option<i32>,
}

impl ModelOfferDto {
    /// Информация о предложении.
    pub fn new() -> ModelOfferDto {
        ModelOfferDto {
            discount: None,
            name: None,
            pos: None,
            pre_discount_price: None,
            price: None,
            region_id: None,
            shipping_cost: None,
            shop_name: None,
            shop_rating: None,
            in_stock: None,
        }
    }
}
