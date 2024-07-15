use anyhow::Result;
use rust_yandexmarket::MarketClient;

#[tokio::main]
async fn main() -> Result<()> {
    let token = std::env::var("MARKET_TOKEN").expect("MARKET_TOKEN must be set");
    let client = MarketClient::new(token)?;
    dbg!(client);
    Ok(())
}
// fn search_in_category(category: CategoryDto, search_string: &str) -> Option<CategoryDto> {
//     if category.name.to_lowercase().contains(&search_string.to_lowercase()) {
//         return Some(category)
//     } else {
//         if let Some(children) = category.children {
//             for category in children {
//                 if let Some(category) = search_in_category(category, search_string) {
//                     return Some(category)
//                 }
//             }
//         }
//     }
//     None
// }