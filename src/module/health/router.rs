use axum::{routing::get, Router};
use crate::module::health::controller::status;

pub fn new() -> Router {
    Router::new()
    .route("/status", get(status))
}