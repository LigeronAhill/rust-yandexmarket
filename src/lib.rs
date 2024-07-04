// lib.rs
//! # rust-yandexmarket
//!
//! Библиотека для работы с API Yandex.Market на языке программирования Rust
mod api_client;
pub use api_client::{MarketClient, BASE_URL};
pub mod apis;
pub mod models;
