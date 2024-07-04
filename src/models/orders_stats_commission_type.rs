/*
 * Партнерский API Маркета
 *
 * API Яндекс Маркета помогает продавцам автоматизировать и упростить работу с маркетплейсом.  В числе возможностей интеграции:  * управление каталогом товаров и витриной,  * обработка заказов,  * изменение настроек магазина,  * получение отчетов.
 *
 * The version of the OpenAPI document: LATEST
 *
 * Generated by: https://openapi-generator.tech
 */

/// OrdersStatsCommissionType : Услуга:  * `FEE` — размещение товара на Маркете. * `FULFILLMENT` — складская обработка. * `LOYALTY_PARTICIPATION_FEE` — участие в программе лояльности и отзывы за баллы, если они подключены. * `AUCTION_PROMOTION` — буст продаж. * `INSTALLMENT` — рассрочка. * `DELIVERY_TO_CUSTOMER` — доставка покупателю. * `EXPRESS_DELIVERY_TO_CUSTOMER` — экспресс-доставка покупателю. * `AGENCY` — прием платежа покупателя. * `PAYMENT_TRANSFER` — перевод платежа покупателя. * `RETURNED_ORDERS_STORAGE` — хранение невыкупов и возвратов. * `SORTING` — обработка заказа. * `INTAKE_SORTING` — организация забора заказов со склада продавца (FBS). * `RETURN_PROCESSING` — обработка заказов на складе (FBY). * `ILLIQUID_GOODS_SALE` — продажа экспроприированных товаров.

/// Услуга:  * `FEE` — размещение товара на Маркете. * `FULFILLMENT` — складская обработка. * `LOYALTY_PARTICIPATION_FEE` — участие в программе лояльности и отзывы за баллы, если они подключены. * `AUCTION_PROMOTION` — буст продаж. * `INSTALLMENT` — рассрочка. * `DELIVERY_TO_CUSTOMER` — доставка покупателю. * `EXPRESS_DELIVERY_TO_CUSTOMER` — экспресс-доставка покупателю. * `AGENCY` — прием платежа покупателя. * `PAYMENT_TRANSFER` — перевод платежа покупателя. * `RETURNED_ORDERS_STORAGE` — хранение невыкупов и возвратов. * `SORTING` — обработка заказа. * `INTAKE_SORTING` — организация забора заказов со склада продавца (FBS). * `RETURN_PROCESSING` — обработка заказов на складе (FBY). * `ILLIQUID_GOODS_SALE` — продажа экспроприированных товаров.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrdersStatsCommissionType {
    #[serde(rename = "FEE")]
    Fee,
    #[serde(rename = "FULFILLMENT")]
    Fulfillment,
    #[serde(rename = "LOYALTY_PARTICIPATION_FEE")]
    LoyaltyParticipationFee,
    #[serde(rename = "AUCTION_PROMOTION")]
    AuctionPromotion,
    #[serde(rename = "INSTALLMENT")]
    Installment,
    #[serde(rename = "DELIVERY_TO_CUSTOMER")]
    DeliveryToCustomer,
    #[serde(rename = "EXPRESS_DELIVERY_TO_CUSTOMER")]
    ExpressDeliveryToCustomer,
    #[serde(rename = "AGENCY")]
    Agency,
    #[serde(rename = "PAYMENT_TRANSFER")]
    PaymentTransfer,
    #[serde(rename = "RETURNED_ORDERS_STORAGE")]
    ReturnedOrdersStorage,
    #[serde(rename = "SORTING")]
    Sorting,
    #[serde(rename = "INTAKE_SORTING")]
    IntakeSorting,
    #[serde(rename = "RETURN_PROCESSING")]
    ReturnProcessing,
    #[serde(rename = "ILLIQUID_GOODS_SALE")]
    IlliquidGoodsSale,
}

impl ToString for OrdersStatsCommissionType {
    fn to_string(&self) -> String {
        match self {
            Self::Fee => String::from("FEE"),
            Self::Fulfillment => String::from("FULFILLMENT"),
            Self::LoyaltyParticipationFee => String::from("LOYALTY_PARTICIPATION_FEE"),
            Self::AuctionPromotion => String::from("AUCTION_PROMOTION"),
            Self::Installment => String::from("INSTALLMENT"),
            Self::DeliveryToCustomer => String::from("DELIVERY_TO_CUSTOMER"),
            Self::ExpressDeliveryToCustomer => String::from("EXPRESS_DELIVERY_TO_CUSTOMER"),
            Self::Agency => String::from("AGENCY"),
            Self::PaymentTransfer => String::from("PAYMENT_TRANSFER"),
            Self::ReturnedOrdersStorage => String::from("RETURNED_ORDERS_STORAGE"),
            Self::Sorting => String::from("SORTING"),
            Self::IntakeSorting => String::from("INTAKE_SORTING"),
            Self::ReturnProcessing => String::from("RETURN_PROCESSING"),
            Self::IlliquidGoodsSale => String::from("ILLIQUID_GOODS_SALE"),
        }
    }
}

impl Default for OrdersStatsCommissionType {
    fn default() -> OrdersStatsCommissionType {
        Self::Fee
    }
}
