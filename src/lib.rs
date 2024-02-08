// lib.rs
//! # rust-yandexmarket
//!
//! Библиотека для работы с API Yandex.Market на языке программирования Rust
mod error;
pub use error::{Error, Result};
mod api_client;
pub use api_client::{MarketClient, BASE_URL};
mod methods;
pub use methods::offer_mapping::{UpdateBusinessOfferPriceDTO, UpdateOfferMappingDTO};
pub use models::outlets::{
    Address, DayOfWeekType, DeliveryRule, Outlet, OutletType, OutletVisibilityType,
    WorkingScheduleItem,
};
pub use models::offer_mappings::{OfferCardStatusType, OfferMappingRequest};

mod models;
