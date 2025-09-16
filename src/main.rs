use std::sync::Arc;

use axum::{
    extract::DefaultBodyLimit, http::{
        header::{AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method
    }, middleware::from_fn, Extension, Router
};
use axum_file_management_service::{
    config::{self, environment::CONFIG, logger}, dto::environment::Environment, middleware::logger_middleware, module::{health, m_file}, state::AppState
};
// use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use tokio::{net::TcpListener, signal};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, cors::CorsLayer, decompression::RequestDecompressionLayer};
use tower_http::limit::RequestBodyLimitLayer;

#[tokio::main]
async fn main() {
    // logger init
    let _logger = logger::main::initialize();

    // env
    let config = &CONFIG;
    let server_url = Environment::get_server_url(config);
    log::info!("service run at {}", server_url.clone());

    // app state
    // let db_url = Environment::get_database_url(&config);
    // let config_state = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    // let pool_async = bb8::Pool::builder().build(config_state).await.unwrap();
    let diesel_pool = config::database::get_diesel_mysql_db_pool();

    let state = AppState { diesel_pool_mysql: Arc::new(diesel_pool), status: "up".to_string() };
    let shared_state = Arc::new(state);

    let cors = CorsLayer::new()
        .allow_origin(["http://localhost:3000".parse::<HeaderValue>().unwrap()])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]);

    let api = Router::new()
        .nest("/health", health::router::new())
        .nest("/m-file", m_file::router::new());

    let router = Router::new()
        .merge(api)
        .layer(cors)
        // Disable the default limit
        .layer(DefaultBodyLimit::disable())
        // Set a different limit
        .layer(RequestBodyLimitLayer::new(
            250 * 1024 * 1024,
        ))
        .route_layer(from_fn(logger_middleware::log_request))
        .layer(Extension(shared_state))
        .layer(
            ServiceBuilder::new()
                .layer(RequestDecompressionLayer::new())
                .layer(CompressionLayer::new()),
        );

    let listener = TcpListener::bind(format!("{}:{}", config.server_host, config.server_port))
        .await
        .unwrap();
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
