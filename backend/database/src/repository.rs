pub mod classes;
pub mod users;

#[cfg(test)]
pub mod test_utils {
    use sqlx::PgPool;

    use crate::DbConnector;

    pub fn test_db_connector(pool: PgPool) -> DbConnector {
        DbConnector::new(pool)
    }
}
