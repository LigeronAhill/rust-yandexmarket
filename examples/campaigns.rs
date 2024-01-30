use rust_yandexmarket::{MarketClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let client = MarketClient::init().await?;
    let campaigns = client.get_all_campaigns().await?;
    let id = campaigns.first().unwrap().id;
    let _campaign = client.get_campaign(id).await?;
    let logins = client.get_logins(id).await?;
    let login = logins.first().unwrap();
    let _login_campaigns = client.get_logins_campaigns(login).await?;
    let _settings = client.get_settings(id).await?;
    Ok(())
}
