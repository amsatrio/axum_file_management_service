use std::{path::PathBuf, sync::Arc};

use axum::{
    body::{Body, Bytes},
    extract::{Extension, Json, Multipart, Path, Query},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::{Datelike, Local};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};
use validator::Validate;

use crate::{
    dto::{
        enumerator::file_type::FileType,
        request::{
            filter_request::Filters, pagination_request::Pagination, search_request::Search,
            sort_request::Sorts,
        },
        response::{
            app_error::AppError, app_response::AppResponse, pagination_response::PaginatedResponse,
        },
    },
    module::m_file::{
        repository,
        schema::{MFile, MFileRequest},
    },
    state::AppState,
};

pub async fn find_by_id(
    Path(id): Path<i64>,
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<MFile>>), AppError> {
    log::info!("status: {}", _state.status);

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

    let result = repository::find_by_id(&mut db_conn, id);
    match result {
        Ok(Some(value)) => {
            let status_code = StatusCode::OK;
            return Ok((
                status_code,
                Json(AppResponse {
                    status: status_code.as_u16(),
                    message: "success".to_owned(),
                    timestamp: chrono::Utc::now().naive_utc(),
                    data: Some(value),
                    error: None
                }),
            ));
        }
        Ok(None) => {
            return Err(AppError::NotFound);
        }
        Err(err) => {
            return Err(err);
        }
    }
}

pub async fn find_all(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<Vec<MFile>>>), AppError> {
    log::info!("status: {}", _state.status);

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

    let result = repository::find_all(&mut db_conn);
    match result {
        Ok(value) => {
            let status_code = StatusCode::OK;
            return Ok((
                status_code,
                Json(AppResponse {
                    status: status_code.as_u16(),
                    message: "success".to_owned(),
                    timestamp: chrono::Utc::now().naive_utc(),
                    data: Some(value),
                    error: None
                }),
            ));
        }
        Err(err) => {
            return Err(err);
        }
    }
}

pub async fn delete_by_id(
    Path(id): Path<i64>,
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<String>>), AppError> {
    log::info!("status: {}", _state.status);

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

    let result = repository::delete_by_id(&mut db_conn, id);
    match result {
        Ok(Some(_)) => {
            let status_code = StatusCode::OK;
            return Ok((
                status_code,
                Json(AppResponse {
                    status: status_code.as_u16(),
                    message: "success".to_owned(),
                    timestamp: chrono::Utc::now().naive_utc(),
                    data: None,
                    error: None
                }),
            ));
        }
        Ok(None) => {
            return Err(AppError::NotFound);
        }
        Err(err) => {
            return Err(err);
        }
    }
}

pub async fn create(
    Extension(_state): Extension<Arc<AppState>>,
    Json(m_file_request): Json<MFileRequest>,
) -> Result<(StatusCode, Json<AppResponse<String>>), AppError> {
    log::info!("status: {}", _state.status);

    let _is_valid = match m_file_request.validate() {
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

    let new_m_file = MFile::from_create_request(m_file_request);
    let existing_biodata_result = repository::find_by_id(&mut db_conn, new_m_file.id);
    match existing_biodata_result {
        Ok(Some(_)) => {
            return Err(AppError::DataExist);
        }
        Ok(None) => {}
        Err(err) => {
            return Err(err);
        }
    };

    let result = repository::insert_mfile(&mut db_conn, new_m_file);

    match result {
        Ok(Some(_)) => {
            let status_code = StatusCode::OK;
            return Ok((
                status_code,
                Json(AppResponse {
                    status: status_code.as_u16(),
                    message: "success".to_owned(),
                    timestamp: chrono::Utc::now().naive_utc(),
                    data: None,
                    error: None
                }),
            ));
        }
        Ok(None) => {
            return Err(AppError::Other(format!("save data failed")).into());
        }
        Err(err) => {
            return Err(err);
        }
    }
}

pub async fn update(
    Extension(_state): Extension<Arc<AppState>>,
    Json(m_file_request): Json<MFileRequest>,
) -> Result<(StatusCode, Json<AppResponse<String>>), AppError> {
    log::info!("status: {}", _state.status);

    let _is_valid = match m_file_request.validate() {
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

    let mut _new_m_file = <MFile>::new(String::new(), String::new(), String::new());
    let existing_biodata_result = repository::find_by_id(&mut db_conn, m_file_request.id.unwrap());
    match existing_biodata_result {
        Ok(None) => {
            return Err(AppError::NotFound);
        }
        Ok(Some(value)) => {
            _new_m_file = <MFile>::from_update_request(m_file_request, value);
        }
        Err(err) => {
            return Err(err);
        }
    };

    let result = repository::update_mfile(&mut db_conn, _new_m_file);

    match result {
        Ok(Some(_)) => {
            let status_code = StatusCode::OK;
            return Ok((
                status_code,
                Json(AppResponse {
                    status: status_code.as_u16(),
                    message: "success".to_owned(),
                    timestamp: chrono::Utc::now().naive_utc(),
                    data: None,
                    error: None
                }),
            ));
        }
        Ok(None) => {
            return Err(AppError::Other(format!("save data failed")).into());
        }
        Err(err) => {
            return Err(err);
        }
    }
}

pub async fn find_page(
    Extension(_state): Extension<Arc<AppState>>,
    Query(_pagination): Query<Pagination>,
    Query(_sort): Query<Sorts>,
    Query(_filter): Query<Filters>,
    Query(_global_search): Query<Search>,
) -> Result<(StatusCode, Json<AppResponse<PaginatedResponse<MFile>>>), AppError> {
    log::info!("status: {}", _state.status);

    if let Err(err) = _pagination.validate() {
        return Err(AppError::InvalidRequest(err).into());
    };
    if let Err(err) = _filter.validate() {
        return Err(AppError::InvalidRequest(err).into());
    };
    if let Err(err) = _sort.validate() {
        return Err(AppError::InvalidRequest(err).into());
    };

    let mut _page = _pagination.page.unwrap_or(0);
    if _page < 0 {
        _page = 0;
    }
    let mut _size = _pagination.size.unwrap_or(5);
    if _size < 1 {
        _size = 1;
    }
    let _filters = _filter._filter.clone().unwrap_or_default();
    let _sorts = _sort._sort.clone().unwrap_or_default();
    let _q = _global_search._q.clone().unwrap_or_default();
    log::info!(
        "page {:?}, size {:?}, filters {:?}, sorts {:?}, global_search {:?}",
        _page,
        _size,
        _filters,
        _sorts,
        _q
    );

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

    let result = repository::pagination(&mut db_conn, _page, _size, _filters, _sorts, _q);
    match result {
        Ok(value) => {
            let mut total_of_pages = value.1 / _size;
            if value.1 % _size != 0 {
                total_of_pages = total_of_pages + 1;
            }

            let status_code = StatusCode::OK;
            let paginated_response = PaginatedResponse {
                content: value.0,
                total_of_elements: value.1,
                total_of_pages: total_of_pages,
            };
            return Ok((
                status_code,
                Json(AppResponse {
                    status: status_code.as_u16(),
                    message: "success".to_owned(),
                    timestamp: chrono::Utc::now().naive_utc(),
                    data: Some(paginated_response),
                    error: None
                }),
            ));
        }
        Err(err) => {
            return Err(err);
        }
    }
}

pub async fn upload(
    Extension(_state): Extension<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<AppResponse<MFile>>), AppError> {
    let mut data: Bytes = <Bytes>::new();
    let mut file_name: String = String::new();
    let mut file_type: String = String::new();

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
        if name == "token".to_string() {
            let token = field.text().await.unwrap();
            log::info!("Token received: {}", token);
            continue;
        }
    }

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
        "data/{}/{}/{:02}/{:02}",
        file_type,
        today.year(),
        today.month(),
        today.day()
    );

    let file_path = format!("{}/{}", date_string, file_name);

    // check existing file
    let result_file_exist = std::fs::exists(file_path.clone());
    if result_file_exist.is_err() {
        return Err(AppError::InternalServerError);
    }
    if result_file_exist.unwrap() {
        log::info!("file exist");
        return Err(AppError::DataExist);
    }

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

    let new_m_file = MFile::new(file_name, file_type, file_path);

    let result = repository::insert_mfile(&mut db_conn, new_m_file.clone());

    match result {
        Ok(_) => {},
        Err(value) => {
            return Err(value);
        },
    };

    let status_code = StatusCode::OK;
    Ok((
        status_code,
        Json(AppResponse {
            status: status_code.as_u16(),
            message: "success".to_owned(),
            timestamp: chrono::Utc::now().naive_utc(),
            data: Some(new_m_file),
            error: None
        }),
    ))
}

pub async fn download(
    Query(id): Query<i64>,
    Extension(_state): Extension<Arc<AppState>>,
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
