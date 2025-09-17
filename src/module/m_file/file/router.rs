use axum::{
    routing::{delete, get, post, put}, Router
};

use crate::module::m_file::file::controller::{copy, delete_file, download, move_file, rename, update, upload};

pub fn new() -> Router {
    Router::new()
        .route("/", post(upload))
        .route("/", put(update))
        .route("/{id}", get(download))
        .route("/{id}", delete(delete_file))
        .route("/rename", put(rename))
        .route("/copy", put(copy))
        .route("/move", put(move_file))
}
