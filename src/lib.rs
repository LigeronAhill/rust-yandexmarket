// lib.rs
//! # rust-yandexmarket
//!
//! Библиотека для работы с API Yandex.Market на языке программирования Rust

use anyhow::Result;
mod models;
#[derive(Debug)]
pub struct MarketClient {
    token: String,
    client: reqwest::Client,
}
impl MarketClient {
    pub fn new<T: ToString>(token: T) -> Result<Self> {
        let token = token.to_string();
        let client = reqwest::Client::builder().gzip(true).build()?;
        Ok(Self { token, client })
    }
}