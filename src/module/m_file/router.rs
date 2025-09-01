use axum::{routing::{get, post}, Router};

use crate::module::m_file::{controller::{create, delete_by_id, find_all, find_by_id, find_page, update}, file};


pub fn new() -> Router {
    Router::new()
    .route("/list", get(find_all))
    .route("/pagination", get(find_page))
    .route("/", post(create).put(update))
    .route("/{id}", get(find_by_id).delete(delete_by_id))
    .nest("/file", file::router::new())
}