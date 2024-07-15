/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// AffectedOrderQualityRatingComponentType : Составляющие индекса качества.  **Для модели DBS:** * `DBS_CANCELLATION_RATE` — доля отмененных товаров. * `DBS_LATE_DELIVERY_RATE` — доля заказов, доставленных после плановой даты.  **Для моделей FBS и Экспресс:** * `FBS_CANCELLATION_RATE` — доля отмененных товаров. * `FBS_LATE_SHIP_RATE` — доля не вовремя отгруженных заказов.

/// Составляющие индекса качества.  **Для модели DBS:** * `DBS_CANCELLATION_RATE` — доля отмененных товаров. * `DBS_LATE_DELIVERY_RATE` — доля заказов, доставленных после плановой даты.  **Для моделей FBS и Экспресс:** * `FBS_CANCELLATION_RATE` — доля отмененных товаров. * `FBS_LATE_SHIP_RATE` — доля не вовремя отгруженных заказов.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AffectedOrderQualityRatingComponentType {
    #[serde(rename = "DBS_CANCELLATION_RATE")]
    DbsCancellationRate,
    #[serde(rename = "DBS_LATE_DELIVERY_RATE")]
    DbsLateDeliveryRate,
    #[serde(rename = "FBS_CANCELLATION_RATE")]
    FbsCancellationRate,
    #[serde(rename = "FBS_LATE_SHIP_RATE")]
    FbsLateShipRate,
}

impl ToString for AffectedOrderQualityRatingComponentType {
    fn to_string(&self) -> String {
        match self {
            Self::DbsCancellationRate => String::from("DBS_CANCELLATION_RATE"),
            Self::DbsLateDeliveryRate => String::from("DBS_LATE_DELIVERY_RATE"),
            Self::FbsCancellationRate => String::from("FBS_CANCELLATION_RATE"),
            Self::FbsLateShipRate => String::from("FBS_LATE_SHIP_RATE"),
        }
    }
}

impl Default for AffectedOrderQualityRatingComponentType {
    fn default() -> AffectedOrderQualityRatingComponentType {
        Self::DbsCancellationRate
    }
}