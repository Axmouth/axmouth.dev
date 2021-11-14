use crate::extra::UserRole;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthSuccess {
    pub token: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BaseResponse<T: Serialize> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub total_results: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetUserProfileResponse {
    pub id: i32,
    pub display_name: String,
    pub created_at: NaiveDateTime,
    pub role: UserRole,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileUploadedDetails {
    pub url: String,
    pub path: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileUploadedResponse {
    pub success: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<FileUploadedDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HealthResponse {
    pub db_up: bool,
    pub db_response_time: Option<String>,
    pub api_up: bool,
    pub api_response_time: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GeolocationDbResponse {
    pub country_code: String,
    pub country_name: String,
    pub city: String,
    pub postal: String,
    pub latitude: String,
    pub state: String,
}
