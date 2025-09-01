
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::util::serializer::date_serializer;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AppResponse<T> {
    pub status: u16,
    pub message: String,
    #[serde(with = "date_serializer")]
    pub timestamp: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<T>,
}