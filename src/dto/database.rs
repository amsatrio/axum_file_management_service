use diesel::prelude::QueryableByName;

#[derive(QueryableByName)]
pub struct CountResult {
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub count: i64,
}
