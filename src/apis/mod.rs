use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

// pub mod bids_api;
// pub mod business_offer_mappings_api;
// pub mod businesses_api;
pub mod campaigns_api;
// pub mod categories_api;
// pub mod chats_api;
// pub mod content_api;
// pub mod dbs_api;
// pub mod delivery_services_api;
// pub mod express_api;
// pub mod fbs_api;
// pub mod fby_api;
// pub mod feed_categories_api;
// pub mod feedbacks_api;
// pub mod feeds_api;
// pub mod goods_stats_api;
// pub mod hidden_offers_api;
// pub mod models_api;
// pub mod offer_mappings_api;
// pub mod offers_api;
// pub mod order_business_information_api;
// pub mod order_delivery_api;
// pub mod order_labels_api;
// pub mod orders_api;
// pub mod orders_stats_api;
// pub mod outlet_licenses_api;
// pub mod outlets_api;
// pub mod price_quarantine_api;
// pub mod prices_api;
// pub mod regions_api;
// pub mod reports_api;
// pub mod returns_api;
// pub mod shipments_api;
// pub mod stocks_api;
// pub mod tariffs_api;
// pub mod warehouses_api;

pub mod configuration;
