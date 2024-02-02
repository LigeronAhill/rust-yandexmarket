use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::FlippingPagerDTO;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionsResponse {
    pub regions: Vec<RegionDTO>,
    pub paging: Option<ForwardScrollingPagerDTO>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionsChildrenResponse {
    pub regions: RegionDTO,
    pub pager: FlippingPagerDTO,
}
/// Регион доставки.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionDTO {
    /// Идентификатор региона.
    pub id: i64,
    /// Название региона.
    pub name: String,
    /// Тип региона.
    #[serde(rename = "type")]
    pub region_type: RegionType,
    /// Информация о родительском регионе.
    /// Указываются родительские регионы до уровня страны.
    pub parent: Option<Box<RegionDTO>>,
    /// Дочерние регионы.
    pub children: Option<Vec<RegionDTO>>,
}
/// Ссылка на следующую страницу.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardScrollingPagerDTO {
    /// Идентификатор следующей страницы результатов.
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}
/// Тип региона.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RegionType {
    /// Неизвестный регион
    Other,
    /// Континент
    Continent,
    /// Регион
    Region,
    /// Страна
    Country,
    /// Область
    CountryDistrict,
    /// Субъект федерации
    Republic,
    /// Крупный город
    City,
    /// Город
    Village,
    /// Район города
    CityDistrict,
    /// Станция метро
    SubwayStation,
    /// Район субъекта федерации
    RepublicArea,
}
impl Display for RegionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
