use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct PaginatedQuery<T> {
    pub data: Option<Vec<T>>,
    pub per_page: i64,
    pub page: i64,
}


#[derive(Deserialize, Serialize)]
pub struct Pagination {
    pub per_page: i64,
    pub page: i64,
}



