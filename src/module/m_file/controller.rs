use std::sync::Arc;

use axum::{
    extract::{Extension, Json, Path, Query},
    http::StatusCode,
};
use validator::Validate;

use crate::{
    dto::{
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
                    error: None,
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
                    error: None,
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
                    error: None,
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
                    error: None,
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

    let mut _new_m_file: MFile;
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
                    error: None,
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
                    error: None,
                }),
            ));
        }
        Err(err) => {
            return Err(err);
        }
    }
}
