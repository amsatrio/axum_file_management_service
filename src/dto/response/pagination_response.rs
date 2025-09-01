use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PaginatedResponse<T> {
    pub total_of_elements: i64,
    pub total_of_pages: i64,
    pub content: Vec<T>,
}
