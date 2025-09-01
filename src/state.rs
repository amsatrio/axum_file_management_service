use std::sync::Arc;

use diesel::{r2d2, MysqlConnection};
// use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};



// #[derive(Debug, Clone)]
pub struct AppState {
    // pub diesel_pool_postgres_async: bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
    pub diesel_pool_mysql: Arc<r2d2::Pool<r2d2::ConnectionManager<MysqlConnection>>>,
    pub status: String
}
