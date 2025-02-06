pub mod error;
pub mod repository;
pub mod types;

pub use repository::classes::{
    ClassEntity, ClassesRepository, InputClassEntity, InputDeleteClassEntity, InputFindClassEntity,
    InputUpdateClassEntity,
};
pub use repository::teachers::{PostgresTeachersRepository, TeacherEntity};
pub use repository::users::{
    InputUpdateUserEntity, InputUserEntity, PostgresUserRepository, UserEntity, UserRepository,
};

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

pub async fn init(url: String) -> Result<DbConnector, Error> {
    let pool = PgPool::connect(&url).await?;
    let db = DbConnector::new(pool);
    db.migration().await?;

    Ok(db)
}

#[cfg(test)]
pub mod tests {
    use std::sync::Arc;

    use super::*;

    use serde::Deserialize;

    #[derive(Deserialize, Debug, Clone)]
    pub struct Config {
        database_url: String,
    }

    pub async fn util_init() -> Result<Arc<DbConnector>, Error> {
        dotenvy::dotenv().ok();
        let config = envy::from_env::<Config>().unwrap();

        // let pool = PgPool::connect(&config.database_url).await.unwrap();
        let db = init(config.database_url).await.unwrap();

        Ok(Arc::new(db))
    }
}
