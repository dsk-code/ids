pub mod classes;
pub mod teachers;
pub mod users;

use derive_new::new;

#[derive(Debug, new)]
pub struct Pagination {
    pub limit: i64,
    pub offset: i64,
}

#[cfg(test)]
pub mod test_utils {
    use sqlx::PgPool;

    use crate::DbConnector;

    pub fn test_db_connector(pool: PgPool) -> DbConnector {
        DbConnector::new(pool)
    }
}
