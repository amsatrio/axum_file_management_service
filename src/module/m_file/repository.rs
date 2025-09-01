use diesel::{dsl::insert_into, prelude::*, sql_query, update};

use crate::{
    diesel_schema::m_file::dsl::*,
    dto::{
        database::CountResult, enumerator::filter_match_mode::FilterMatchMode, request::{filter_request::Filter, sort_request::Sort}, response::app_error::AppError
    },
    module::m_file::schema::MFile,
    util::string_manipulation,
};

pub fn find_by_id(
    conn: &mut MysqlConnection,
    mfile_id: i64,
) -> Result<Option<MFile>, AppError> {
    let user = m_file
        .filter(id.eq(mfile_id))
        .select(MFile::as_select())
        .first::<MFile>(conn)
        .optional()
        .map_err(|error| AppError::Other(format!("query failed: {}, id: {}", error, mfile_id)))?;

    Ok(user)
}

pub fn find_query_by_id(
    conn: &mut MysqlConnection,
    mfile_id: i64,
) -> Result<Option<MFile>, AppError> {
    let query = "SELECT * 
            FROM m_file 
            WHERE id = ?";

    let user: Option<MFile> = sql_query(query)
        .bind::<diesel::sql_types::BigInt, _>(mfile_id)
        .get_result::<MFile>(conn)
        .optional()
        .map_err(|error| AppError::Other(format!("query failed: {}, id: {}", error, mfile_id)))?;
    Ok(user)
}

pub fn find_all(conn: &mut MysqlConnection) -> Result<Vec<MFile>, AppError> {
    let query = "SELECT * 
            FROM m_file";

    let user: Vec<MFile> = sql_query(query)
        .get_results::<MFile>(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    Ok(user)
}

pub fn delete_by_id(conn: &mut MysqlConnection, mfile_id: i64) -> Result<Option<()>, AppError> {
    let rows_affected = diesel::delete(m_file.filter(id.eq(mfile_id)))
        .execute(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}, id: {}", error, mfile_id)))?;

    if rows_affected > 0 {
        return Ok(Some(()));
    }
    return Ok(None);
}

pub fn insert_mfile(
    conn: &mut MysqlConnection,
    mfile: MFile,
) -> Result<Option<()>, AppError> {
    let rows_affected = insert_into(m_file)
        .values(&mfile)
        .execute(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    if rows_affected > 0 {
        return Ok(Some(()));
    }
    return Ok(None);
}

pub fn update_mfile(
    conn: &mut MysqlConnection,
    mfile: MFile,
) -> Result<Option<()>, AppError> {
    let rows_affected = update(m_file.filter(id.eq(mfile.id)))
        .set((
            modified_by.eq(mfile.modified_by),
            modified_on.eq(mfile.modified_on),
            deleted_by.eq(mfile.deleted_by),
            deleted_on.eq(mfile.deleted_on),
            is_delete.eq(mfile.is_delete),
            file_name.eq(mfile.file_name),
            file_type.eq(mfile.file_type),
            file.eq(mfile.file),
            file_path.eq(mfile.file_path),
        ))
        .execute(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}, id: {}", error, mfile.id)))?;
    if rows_affected > 0 {
        return Ok(Some(()));
    }
    return Ok(None);
}

pub fn pagination(
    conn: &mut MysqlConnection,
    page: i64,
    size: i64,
    filters: Vec<Filter>,
    sorts: Vec<Sort>,
    search: String,
) -> Result<(Vec<MFile>, i64), AppError> {
    // Build the query
    let mut query = "SELECT *".to_string();
    let mut query_count = "SELECT COUNT(*) AS count".to_string();
    let query_table = "FROM m_file".to_string();

    // Sort
    let mut query_sort = String::new();
    for sort in sorts {
        query_sort = "ORDER BY".to_owned();
        let sort_asc = if sort.desc {
            "DESC".to_string()
        } else {
            "ASC".to_string()
        };
        let sort_id = string_manipulation::cleanse_string(&sort.id);
        query_sort = format!("{} {} {}", query_sort, sort_id, sort_asc);
        break;
    }

    // Search
    let mut query_search = String::new();
    if search != String::new() {
        query_search = format!(
            "WHERE fullname LIKE '%{}%'",
            string_manipulation::cleanse_string(&search)
        );
    }

    // Filter
    let mut query_filter = "".to_string();
    for filter in filters {
        let filter_id = string_manipulation::cleanse_string(&filter.id);
        let filter_value = string_manipulation::cleanse_string(&filter.value);
        let mut filter_query_temp = "".to_string();
        match filter.match_mode {
            FilterMatchMode::CONTAINS => {
                filter_query_temp = format!("{} LIKE '%{}%'", filter_id, filter_value);
            }
            FilterMatchMode::SW => {
                filter_query_temp = format!("{} LIKE '{}%'", filter_id, filter_value);
            }
            FilterMatchMode::EW => {
                filter_query_temp = format!("{} LIKE '%{}'", filter_id, filter_value);
            }
            FilterMatchMode::BETWEEN => {}
            FilterMatchMode::EQUALS => {
                filter_query_temp = format!("{} = '{}'", filter_id, filter_value);
            }
            FilterMatchMode::NOT => {
                filter_query_temp = format!("{} <> '{}'", filter_id, filter_value);
            }
            FilterMatchMode::LT => {
                filter_query_temp = format!("{} < '{}'", filter_id, filter_value);
            }
            FilterMatchMode::GT => {
                filter_query_temp = format!("{} > '{}'", filter_id, filter_value);
            }
        };

        if query_search == "".to_owned() {
            query_search = " ".to_owned();
            query_filter = format!("WHERE {}", filter_query_temp);
        } else {
            query_filter = format!("{} AND {}", query_filter, filter_query_temp);
        }
    }

    // Pagination
    let query_pagination = format!("LIMIT {} OFFSET {}", size, size * (page));

    // Final
    query = format!(
        "{} {} {} {} {} {}",
        query, query_table, query_search, query_filter, query_sort, query_pagination
    );
    query_count = format!(
        "{} {} {} {} {}",
        query_count, query_table, query_search, query_filter, query_sort
    );
    log::info!(
        "repository > find_diesel_query_mfile_page > query: {:#?}",
        query
    );
    log::info!(
        "repository > find_diesel_query_mfile_page > query_count: {:#?}",
        query_count
    );

    let data_vec: Vec<MFile> = sql_query(query)
        .get_results::<MFile>(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    let results = sql_query(query_count)
        .load::<CountResult>(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    Ok((data_vec, results[0].count))
}
