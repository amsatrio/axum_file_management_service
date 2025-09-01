use std::sync::Arc;

use axum::{http::StatusCode, Extension, Json};

use crate::{dto::response::{app_error::AppError, app_response::AppResponse}, state::AppState};

pub async fn status(Extension(_state): Extension<Arc<AppState>>) -> Result<(StatusCode, Json<AppResponse<String>>), AppError> {
    log::info!("status: {}", _state.status);
    let status_code = StatusCode::OK;
    Ok((
        status_code,
        Json(AppResponse {
            status: status_code.as_u16(),
            message: "success".to_owned(),
            timestamp: chrono::Utc::now().naive_utc(),
            data: Some(_state.status.clone()),
            error: None
        }),
    ))
}