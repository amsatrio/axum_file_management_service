use std::{path::PathBuf, sync::Arc};

use axum::{
    body::{Body, Bytes},
    extract::{Extension, Json, Multipart, Path},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::{Datelike, Local};
use tokio::{
    fs::{File, remove_file},
    io::{AsyncReadExt, AsyncWriteExt},
};
use validator::Validate;

use crate::{
    dto::{
        enumerator::file_type::FileType,
        response::{app_error::AppError, app_response::AppResponse},
    },
    module::m_file::{
        repository,
        schema::{MFile, MFileRequest},
    },
    state::AppState,
};

pub async fn create(
    Extension(_state): Extension<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<AppResponse<MFile>>), AppError> {
    let mut data: Bytes = <Bytes>::new();
    let mut file_name: String = String::new();
    let mut file_type: String = String::new();
    let mut _m_file_request: MFileRequest = MFileRequest::new(None, None, None, None, None, None);

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        if name == "file".to_string() {
            file_name = field.file_name().unwrap_or("").to_string();
            file_type = field.content_type().unwrap_or("").to_string();
            data = field.bytes().await.unwrap();

            let category = match file_type.as_str() {
                // Image types
                "image/jpeg" | "image/png" | "image/gif" => FileType::IMAGE,
                // Audio types
                "audio/mpeg" | "audio/wav" | "audio/ogg" => FileType::AUDIO,
                // Video types
                "video/mp4" | "video/x-msvideo" | "video/x-flv" => FileType::VIDEO,
                // Document types
                "application/pdf"
                | "application/msword"
                | "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => {
                    FileType::DOCUMENT
                }
                // Default case for unknown types
                _ => FileType::UNKNOWN,
            };
            file_type = category.to_string();

            continue;
        }
        if name == "payload".to_string() {
            let payload_tmp = field
                .text()
                .await
                .map_err(|e| AppError::Other(e.to_string()))?;

                _m_file_request = serde_json::from_str(&payload_tmp).map_err(|e| AppError::Other(e.to_string()))?;
            continue;
        }
    }

    let _is_valid = match _m_file_request.validate() {
        Ok(value) => value,
        Err(error) => {
            return Err(AppError::InvalidRequest(error).into());
        }
    };

    // get db connection
    let db_conn_result = _state.diesel_pool_mysql.get();
    let mut db_conn;
    match db_conn_result {
        Ok(value) => {
            db_conn = value;
        }
        Err(error) => {
            return Err(AppError::Other(format!("get connection failed {error}")).into());
        }
    };

    let today = Local::now();
    let date_string = format!(
        "{}/{}/{:02}/{:02}",
        file_type,
        today.year(),
        today.month(),
        today.day()
    );

    let file_path = format!("/data/{}/{}/{}/{}", _m_file_request.module_name.clone().unwrap_or("public".to_string()), _m_file_request.user_id.unwrap_or(0), date_string, file_name);

    // check existing file
    let result_file_exist = std::fs::exists(file_path.clone());
    if result_file_exist.is_err() {
        return Err(AppError::InternalServerError);
    }
    if result_file_exist.unwrap() {
        log::info!("file exist");
        return Err(AppError::DataExist);
    }

    // check existing data
    let _existing_data_result = repository::find_by_id(&mut db_conn, _m_file_request.id.unwrap());
    match _existing_data_result {
        Ok(None) => {}
        Ok(Some(_)) => {
            return Err(AppError::DataExist);
        }
        Err(_error) => {
            if _error != AppError::DataExist {
                return Err(_error);
            }
        }
    };

    let status_create_dir = std::fs::create_dir_all(date_string);
    if status_create_dir.is_err() {
        log::info!("create dir failed");
        return Err(AppError::InternalServerError);
    }

    // create file
    let file = File::create(&file_path).await.map_err(|e| {
        log::error!("Failed to create file: {}", e);
    });
    if file.is_err() {
        return Err(AppError::InternalServerError);
    }

    // save data to file
    let status_write_data = file.unwrap().write_all(&data).await.map_err(|e| {
        log::error!("Failed to write to file: {}", e);
    });
    if status_write_data.is_err() {
        return Err(AppError::InternalServerError);
    }

    let mut new_m_file = MFile::from_create_request(_m_file_request);
    new_m_file.file_path = Some(file_path);
    new_m_file.file_type = Some(file_type);
    new_m_file.file_name = Some(file_name);

    let result = repository::insert_mfile(&mut db_conn, new_m_file.clone());

    match result {
        Ok(_) => {}
        Err(value) => {
            return Err(value);
        }
    };

    let status_code = StatusCode::OK;
    Ok((
        status_code,
        Json(AppResponse {
            status: status_code.as_u16(),
            message: "success".to_owned(),
            timestamp: chrono::Utc::now().naive_utc(),
            data: Some(new_m_file),
            error: None,
        }),
    ))
}

pub async fn update(
    Extension(_state): Extension<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<AppResponse<MFile>>), AppError> {
    let mut data: Bytes = <Bytes>::new();
    let mut file_name: String = String::new();
    let mut file_type: String = String::new();
    let mut _m_file_request: MFileRequest = MFileRequest::new(None, None, None, None, None, None);

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        if name == "file".to_string() {
            file_name = field.file_name().unwrap_or("").to_string();
            file_type = field.content_type().unwrap_or("").to_string();
            data = field.bytes().await.unwrap();

            let category = match file_type.as_str() {
                // Image types
                "image/jpeg" | "image/png" | "image/gif" => FileType::IMAGE,
                // Audio types
                "audio/mpeg" | "audio/wav" | "audio/ogg" => FileType::AUDIO,
                // Video types
                "video/mp4" | "video/x-msvideo" | "video/x-flv" => FileType::VIDEO,
                // Document types
                "application/pdf"
                | "application/msword"
                | "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => {
                    FileType::DOCUMENT
                }
                // Default case for unknown types
                _ => FileType::UNKNOWN,
            };
            file_type = category.to_string();

            continue;
        }
        if name == "payload".to_string() {
            let payload_tmp = field
                .text()
                .await
                .map_err(|e| AppError::Other(e.to_string()))?;

                _m_file_request = serde_json::from_str(&payload_tmp).map_err(|e| AppError::Other(e.to_string()))?;
            continue;
        }
    }

    let _is_valid = match _m_file_request.validate() {
        Ok(value) => value,
        Err(error) => {
            return Err(AppError::InvalidRequest(error).into());
        }
    };

    // get db connection
    let db_conn_result = _state.diesel_pool_mysql.get();
    let mut db_conn;
    match db_conn_result {
        Ok(value) => {
            db_conn = value;
        }
        Err(error) => {
            return Err(AppError::Other(format!("get connection failed {error}")).into());
        }
    };

    let today = Local::now();
    let date_string = format!(
        "{}/{}/{:02}/{:02}",
        file_type,
        today.year(),
        today.month(),
        today.day()
    );

    let file_path = format!("/data/{}/{}/{}/{}", _m_file_request.module_name.clone().unwrap_or("public".to_string()), _m_file_request.user_id.unwrap_or(0), date_string, file_name);

    // check existing file
    let result_file_exist = std::fs::exists(file_path.clone());
    if result_file_exist.is_err() {
        return Err(AppError::InternalServerError);
    }
    if result_file_exist.unwrap() {
        // remove file
        let file_path_buf = PathBuf::from(file_path.clone());
        let _remove_file_result = remove_file(file_path_buf.clone())
            .await
            .map_err(|error| AppError::Other(format!("remove file failed: {}", error)))?;
    }

    // check existing data
    let mut _existing_data = MFile::new(String::new(), String::new(), String::new());
    let _existing_data_result = repository::find_by_id(&mut db_conn, _m_file_request.id.unwrap());
    match _existing_data_result {
        Ok(None) => {
            return Err(AppError::NotFound);
        }
        Ok(Some(_value)) => {
            _existing_data = _value;
        }
        Err(_error) => {
            if _error != AppError::DataExist {
                return Err(_error);
            }
        }
    };

    let status_create_dir = std::fs::create_dir_all(date_string);
    if status_create_dir.is_err() {
        log::info!("create dir failed");
        return Err(AppError::InternalServerError);
    }

    // create file
    let file = File::create(&file_path).await.map_err(|e| {
        log::error!("Failed to create file: {}", e);
    });
    if file.is_err() {
        return Err(AppError::InternalServerError);
    }

    // save data to file
    let status_write_data = file.unwrap().write_all(&data).await.map_err(|e| {
        log::error!("Failed to write to file: {}", e);
    });
    if status_write_data.is_err() {
        return Err(AppError::InternalServerError);
    }

    let mut new_m_file = MFile::from_update_request(_m_file_request, _existing_data);
    new_m_file.file_path = Some(file_path);
    new_m_file.file_type = Some(file_type);
    new_m_file.file_name = Some(file_name);

    let result = repository::update_mfile(&mut db_conn, new_m_file.clone());

    match result {
        Ok(_) => {}
        Err(value) => {
            return Err(value);
        }
    };

    let status_code = StatusCode::OK;
    Ok((
        status_code,
        Json(AppResponse {
            status: status_code.as_u16(),
            message: "success".to_owned(),
            timestamp: chrono::Utc::now().naive_utc(),
            data: Some(new_m_file),
            error: None,
        }),
    ))
}

pub async fn find_by_id(
    Extension(_state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    // find path file by id
    let mut _file_path_string = String::new();
    let mut _file_name = String::new();
    // get db connection
    let db_conn_result = _state.diesel_pool_mysql.get();
    let mut db_conn;
    match db_conn_result {
        Ok(value) => {
            db_conn = value;
        }
        Err(error) => {
            return Err(AppError::Other(format!("get connection failed {error}, id: {id}")).into());
        }
    };
    let find_by_id_result = repository::find_by_id(&mut db_conn, id);
    match find_by_id_result {
        Ok(Some(value)) => {
            _file_path_string = value.file_path.unwrap_or(String::new());
            _file_name = value.file_name.unwrap_or(String::new());
        }
        Ok(None) => {
            return Err(AppError::NotFound);
        }
        Err(error) => {
            return Err(error);
        }
    };

    let file_path = PathBuf::from(_file_path_string);

    let open_file_response = match File::open(&file_path).await {
        Ok(mut file) => {
            let mut contents = Vec::new();
            if let Err(_) = file.read_to_end(&mut contents).await {
                return Err(AppError::Other(format!("failed to read file")).into());
            }

            let response_builder: axum::http::Response<Body> = axum::http::Response::builder()
                .header(
                    "Content-Disposition",
                    format!("attachment; filename=\"{}\"", _file_name),
                )
                .header("Content-Type", "application/octet-stream")
                .body(contents.into())
                .unwrap();

            Ok(response_builder)
        }
        Err(_) => Err(AppError::NotFound),
    };

    return open_file_response;
}

pub async fn delete_by_id(
    Extension(_state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<(StatusCode, Json<AppResponse<String>>), AppError> {
    // find path file by id
    let mut _file_path_string = String::new();
    let mut _file_name = String::new();
    // get db connection
    let db_conn_result = _state.diesel_pool_mysql.get();
    let mut db_conn;
    match db_conn_result {
        Ok(value) => {
            db_conn = value;
        }
        Err(error) => {
            return Err(AppError::Other(format!("get connection failed {error}, id: {id}")).into());
        }
    };
    let find_by_id_result = repository::find_by_id(&mut db_conn, id);
    match find_by_id_result {
        Ok(Some(value)) => {
            _file_path_string = value.file_path.unwrap_or(String::new());
            _file_name = value.file_name.unwrap_or(String::new());
        }
        Ok(None) => {
            return Err(AppError::NotFound);
        }
        Err(error) => {
            return Err(error);
        }
    };

    let _delete_result = repository::delete_by_id(&mut db_conn, id);
    match _delete_result {
        Ok(_) => {}
        Err(error) => {
            return Err(error.into());
        }
    };

    let file_path = PathBuf::from(_file_path_string);

    let _remove_file_result = remove_file(file_path.clone())
        .await
        .map_err(|error| AppError::Other(format!("remove file failed: {}", error)))?;

    let status_code = StatusCode::OK;
    return Ok((
        status_code,
        Json(AppResponse::<String> {
            status: status_code.as_u16(),
            message: "success".to_owned(),
            timestamp: chrono::Utc::now().naive_utc(),
            data: None,
            error: None,
        }),
    ));
}
