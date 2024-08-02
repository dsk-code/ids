// use ids_auth as auth;
use ids_database::DbConnector;

pub mod error;
pub mod router;

use error::Error;
use sqlx::PgPool;
use std::sync::Arc;

pub struct State {
    db: Arc<DbConnector>,
}

impl State {
    pub fn new(db: DbConnector) -> Self {
        Self { db: Arc::new(db) }
    }

    pub fn db(&self) -> Arc<DbConnector> {
        self.db.clone()
    }
}

pub async fn init(pool: PgPool) -> Result<State, Error> {
    let db = State::new(DbConnector::new(pool));
    Ok(db)
}
