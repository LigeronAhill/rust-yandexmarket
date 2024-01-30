use rust_yandexmarket::{MarketClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let client = MarketClient::init().await?;
    for c in client.campaigns() {
        println!("{:#?}", c.business);
    }
    Ok(())
}
