use diesel::{r2d2, MysqlConnection};

use crate::config::environment::CONFIG;

/// Initialize database connection pool based on `DATABASE_URL` environment variable.
///
/// See more: <https://docs.rs/diesel/latest/diesel/r2d2/index.html>.
pub fn get_diesel_mysql_db_pool() -> r2d2::Pool<r2d2::ConnectionManager<MysqlConnection>> {
    let config_env = &CONFIG;
    let database_url = config_env.get_database_url();
    let manager = r2d2::ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::builder()
        .max_size(config_env.database_max_pool)
        .min_idle(Some(config_env.database_min_pool))
        .idle_timeout(Some(std::time::Duration::from_secs(10)))
        .build(manager)
        .expect("connection db failed")
}
