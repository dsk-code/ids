pub mod error;
pub mod repository;

pub use repository::users::UserRepository;

use error::Error;
use sqlx::PgPool;

pub struct DbConnector {
    pool: PgPool,
}

impl DbConnector {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn get_pool(&self) -> PgPool {
        self.pool.clone()
    }

    pub async fn migration(&self) -> Result<(), Error> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .map_err(Error::MigrationError)?;
        Ok(())
    }
}

pub async fn init(pool: PgPool) -> Result<DbConnector, Error> {
    let db = DbConnector::new(pool);
    db.migration().await?;

    Ok(db)
}
