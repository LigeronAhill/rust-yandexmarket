fn main() {
    let token = std::env::var("MARKET_TOKEN").expect("MARKET_TOKEN must be set");
    let mut config = openapi::apis::configuration::Configuration::new();
    let _ = config.bearer_access_token.insert(token);
    println!("{:?}", config);
}