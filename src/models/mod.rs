use serde::{Deserialize, Serialize};

pub mod campaigns;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiResponseStatusType {
    OK,
    ERROR,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorDTO {
    pub code: String,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: Option<ApiResponseStatusType>,
    pub error: Option<Vec<ApiErrorDTO>>,
}
