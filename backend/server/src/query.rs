use ids_database as db;
use serde::Deserialize;

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
