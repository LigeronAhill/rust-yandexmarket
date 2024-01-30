// lib.rs
//! # rust-yandexmarket
//!
//! Библиотека для работы с API Yandex.Market на языке программирования Rust
mod error;
pub use error::{Error, Result};
mod api_client;
pub use api_client::MarketClient;
mod methods;
mod models;
