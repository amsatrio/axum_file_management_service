use axum::{
    routing::{delete, get, post, put}, Router
};

use crate::module::m_file::file::controller::{create, delete_by_id, find_by_id, update};

pub fn new() -> Router {
    Router::new()
        .route("/", post(create))
        .route("/", put(update))
        .route("/{id}", get(find_by_id))
        .route("/{id}", delete(delete_by_id))
}
