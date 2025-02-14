use ids_database as db;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct Paginations {
    pub offset: i64,
    pub limit: i64,
}

impl From<Paginations> for db::Pagination {
    fn from(page: Paginations) -> Self {
        db::Pagination::new(page.limit, page.offset)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct PaginationsWithTeachersStatus {
    pub offset: i64,
    pub limit: i64,
    pub status: String,
}

impl From<PaginationsWithTeachersStatus> for db::Pagination {
    fn from(page: PaginationsWithTeachersStatus) -> Self {
        db::Pagination::new(page.limit, page.offset)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequestGetAddress {
    pub post_code: String,
}
