# rust-yandexmarket

## Библиотека для работы с API Yandex.Market на языке программирования Rust

## Usage

```toml
[dependencies]
rust-yandexmarket = "0.1.0"
```

## Examples

```rust
use rust_yandexmarket::{MarketClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let market_client = MarketClient::init().await?;
    // Use the market_client instance...
    Ok(())
}

```
## Config.toml

```toml
[config]
access_token = 'someaccesstoken'
check_token = 'somechecktoken'
```
