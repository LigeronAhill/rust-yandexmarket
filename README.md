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
    let client = MarketClient::init().await?;
    for c in client.campaigns() {
        println!("{:#?}", c.business);
    }
    Ok(())
}

```
