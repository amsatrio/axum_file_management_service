use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable, QueryableByName};
use diesel::Selectable;
use serde::{Deserialize, Serialize};

use validator::Validate;

use crate::diesel_schema::m_file;
use crate::util::serializer::{date_serializer, option_date_serializer};

#[derive(
    Debug,
    Deserialize,
    Serialize,
    Clone,
    Queryable,
    QueryableByName,
    Insertable,
    Selectable
)]
#[diesel(table_name = m_file)]
pub struct MFile {
    pub id: i64,
    pub file_name: Option<String>,
    pub file_type: Option<String>,
    pub file_path: Option<String>,
    pub file_size: Option<String>,
    pub module_id: Option<i64>,
    pub created_by: i64,
    #[serde(with = "date_serializer")]
    pub created_on: NaiveDateTime,
    pub modified_by: Option<i64>,
    #[serde(with = "option_date_serializer")]
    pub modified_on: Option<NaiveDateTime>,
    pub deleted_by: Option<i64>,
    #[serde(with = "option_date_serializer")]
    pub deleted_on: Option<NaiveDateTime>,
    pub is_delete: bool,
}

impl MFile {
    pub fn new(file_name: String, file_type: String, file_path: String, file_size: String, module_id: i64, user_id: i64) -> MFile {
        let date_now = chrono::Utc::now().naive_utc();
        MFile {
            id: date_now.and_utc().timestamp(),
            file_name: Some(file_name),
            file_type: Some(file_type),
            file_path: Some(file_path),
            file_size: Some(file_size),
            module_id: Some(module_id),
            created_by: user_id,
            created_on: date_now,
            modified_by: None,
            modified_on: None,
            deleted_by: None,
            deleted_on: None,
            is_delete: false,
        }
    }
    pub fn from_create_request(request: MFileRequest) -> MFile {
        let date_now = chrono::Utc::now().naive_utc();
        let is_delete = request.is_delete.unwrap_or(false);
        let mut deleted_by: Option<i64> = None;
        let mut deleted_on: Option<NaiveDateTime> = None;
        if is_delete {
            deleted_by = request.user_id;
            deleted_on = Some(date_now);
        }
        MFile {
            id: request.id.unwrap(),
            file_name: request.file_name,
            file_type: request.file_type,
            file_path: request.file_path,
            file_size: request.file_size,
            module_id: request.module_id,
            created_by: request.user_id.unwrap_or(0),
            created_on: date_now,
            modified_by: None,
            modified_on: None,
            deleted_by: deleted_by,
            deleted_on: deleted_on,
            is_delete: is_delete,
        }
    }
    pub fn from_update_request(request: MFileRequest, existing: MFile) -> MFile {
        let date_now = chrono::Utc::now().naive_utc();
        let is_delete = request.is_delete.unwrap_or(false);
        let mut deleted_by: Option<i64> = None;
        let mut deleted_on: Option<NaiveDateTime> = None;
        if is_delete {
            deleted_by = request.user_id;
            deleted_on = Some(date_now);
        }
        MFile {
            id: request.id.unwrap(),
            file_name: request.file_name,
            file_type: request.file_type,
            file_path: request.file_path,
            file_size: request.file_size,
            module_id: request.module_id,
            created_by: existing.created_by,
            created_on: existing.created_on,
            modified_by: request.user_id,
            modified_on: Some(date_now),
            deleted_by: deleted_by,
            deleted_on: deleted_on,
            is_delete: is_delete,
        }
    }
}



#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct MFileRequest {
    #[validate(
        range(min = 1, max = 999999, message = "must be between 1-999999 chars"),
        required(message = "mandatory")
    )]
    pub id: Option<i64>,
    #[validate(
        length(min = 3, message = "must be greater than 3 chars"),
        required(message = "mandatory")
    )]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<String>,
    #[validate(required(message = "mandatory"))]
    pub is_delete: Option<bool>,
    #[validate(required(message = "mandatory"))]
    pub module_id: Option<i64>,
    #[validate(required(message = "mandatory"))]
    pub user_id: Option<i64>,
}

impl MFileRequest {
    pub fn new(id: Option<i64>, file_name: Option<String>, file_type: Option<String>, file_path: Option<String>, module_id: Option<i64>, user_id: Option<i64>) -> MFileRequest {
        MFileRequest {
            id: id,
            file_name: file_name,
            file_type: file_type,
            file_path: file_path,
            file_size: None,
            is_delete: Some(false),
            module_id: module_id,
            user_id: user_id
        }
    }
}


#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct MFileRenameRequest {
    #[validate(
        range(min = 1, max = 999999, message = "must be between 1-999999 chars"),
        required(message = "mandatory")
    )]
    pub id: Option<i64>,
    #[validate(
        length(min = 3, message = "must be greater than 3 chars"),
        required(message = "mandatory")
    )]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct MFileCopyMoveRequest {
    #[validate(
        range(min = 1, max = 999999, message = "must be between 1-999999 chars"),
        required(message = "mandatory")
    )]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}


#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct MFileResponse {
    #[validate(
        range(min = 1, max = 999999, message = "must be between 1-999999 chars"),
        required(message = "mandatory")
    )]
    pub id: Option<i64>,
    #[validate(
        length(min = 3, message = "must be greater than 3 chars"),
        required(message = "mandatory")
    )]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<String>,
    #[validate(required(message = "mandatory"))]
    pub module_id: Option<i64>,
}