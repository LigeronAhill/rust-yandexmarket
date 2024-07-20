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

/// OfferRecommendationInfoDto : Рекомендации, касающиеся цены на товар.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OfferRecommendationInfoDto {
    /// Ваш SKU — идентификатор товара в вашей системе.  Разрешена любая последовательность длиной до 255 знаков.  Правила использования SKU:  * У каждого товара SKU должен быть свой.  * SKU товара нельзя менять — можно только удалить товар и добавить заново с новым SKU.  * Уже заданный SKU нельзя освободить и использовать заново для другого товара. Каждый товар должен получать новый идентификатор, до того никогда не использовавшийся в вашем каталоге.  [Что такое SKU и как его назначать](https://yandex.ru/support/marketplace/assortment/add/index.html#fields)
    #[serde(rename = "offerId", skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[serde(
        rename = "recommendedCofinancePrice",
        skip_serializing_if = "Option::is_none"
    )]
    pub recommended_cofinance_price: Option<Box<crate::models::BasePriceDto>>,
    #[serde(
        rename = "competitivenessThresholds",
        skip_serializing_if = "Option::is_none"
    )]
    pub competitiveness_thresholds: Option<Box<crate::models::PriceCompetitivenessThresholdsDto>>,
}

impl OfferRecommendationInfoDto {
    /// Рекомендации, касающиеся цены на товар.
    pub fn new() -> OfferRecommendationInfoDto {
        OfferRecommendationInfoDto {
            offer_id: None,
            recommended_cofinance_price: None,
            competitiveness_thresholds: None,
        }
    }
}
