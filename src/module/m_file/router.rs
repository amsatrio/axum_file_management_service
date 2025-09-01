use axum::{routing::{get, post}, Router};

use crate::module::m_file::controller::{create, delete_by_id, download, find_all, find_by_id, find_page, update, upload};


pub fn new() -> Router {
    Router::new()
    .route("/list", get(find_all))
    .route("/pagination", get(find_page))
    .route("/", post(create).put(update))
    .route("/{biodata_id}", get(find_by_id).delete(delete_by_id))
    .route("/download", get(download))
    .route("/upload", post(upload))
}