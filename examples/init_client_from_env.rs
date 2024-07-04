use anyhow::Result;
use rust_yandexmarket::MarketClient;

#[tokio::main]
async fn main() -> Result<()> {
    let client = MarketClient::from_env().await?;
    println!("Got {} campaigns", client.campaigns().len());
    for campaign in client.campaigns() {
        println!("{campaign:#?}")
    }
    Ok(())
}
