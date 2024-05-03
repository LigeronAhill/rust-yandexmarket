use rust_yandexmarket::MarketClient;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let client = MarketClient::init().await?;
    let campaigns = client.campaigns().get_all_campaigns().await?;
    let id = campaigns.first().unwrap().id;
    let _campaign = client.campaigns().get_campaign(id).await?;
    let logins = client.campaigns().get_logins(id).await?;
    let login = logins.first().unwrap();
    let _login_campaigns = client.campaigns().get_logins_campaigns(login).await?;
    let settings = client.campaigns().get_settings(id).await?;
    println!("{settings:#?}");
    Ok(())
}
