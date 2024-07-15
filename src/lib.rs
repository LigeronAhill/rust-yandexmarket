// lib.rs
//! # rust-yandexmarket
//!
//! Библиотека для работы с API Yandex.Market на языке программирования Rust

use anyhow::Result;
use reqwest::Url;

use crate::models::{CategoryDto, CategoryParameterDto, GetCategoriesResponse, GetCategoryContentParametersResponse};

pub mod models;

#[derive(Debug)]
pub struct MarketClient {
    token: String,
    base_url: Url,
    client: reqwest::Client,
}
impl MarketClient {
    pub fn new<T: ToString>(token: T) -> Result<Self> {
        let token = token.to_string();
        let base_url = Url::parse("https://api.partner.market.yandex.ru")?;
        let client = reqwest::Client::builder().gzip(true).build()?;
        Ok(Self { token, base_url, client })
    }
    pub async fn get_categories(&self) -> Result<Vec<CategoryDto>> {
        let uri = self.base_url.join("categories/tree")?;
        let result: GetCategoriesResponse = self
            .client
            .post(uri)
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;
        Ok(result.result.and_then(|r| r.children).unwrap_or_default())
    }
    pub async fn get_category_parameters(&self, category_id: i64) -> Result<Vec<CategoryParameterDto>> {
        let endpoint = format!("category/{category_id}/parameters");
        let uri = self.base_url.join(&endpoint)?;
        let response: GetCategoryContentParametersResponse = self
            .client
            .post(uri)
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;
        let result = response.result.and_then(|r| r.parameters).unwrap_or_default();
        Ok(result)
    }
}