// use ids_auth as auth;
use ids_database::DbConnector;

pub mod error;
pub mod middleware;
pub mod model;
pub mod router;

use ids_auth::{key_init, types::KeyInitConfig};

use error::Error;
use shuttle_runtime::SecretStore;
use sqlx::PgPool;
use std::sync::Arc;

pub struct State {
    db: Arc<DbConnector>,
    secret_store: SecretStore,
}

impl State {
    pub fn new(db: DbConnector, secret_store: SecretStore) -> Self {
        Self {
            db: Arc::new(db),
            secret_store,
        }
    }

    pub fn db(&self) -> Arc<DbConnector> {
        self.db.clone()
    }
}

pub async fn init(secret_store: SecretStore, pool: PgPool) -> Result<State, Error> {
    let auth_secret = KeyInitConfig {
        access_token_url: secret_store
            .get("ACCESS_TOKEN_URL")
            .ok_or(error::Error::NotFoundSecrets("ACCESS_TOKEN_URL".into()))?,
        management_api_client_id: secret_store.get("MANAGEMENT_API_CLIENT_ID").ok_or(
            error::Error::NotFoundSecrets("MANAGEMENT_API_CLIENT_ID".into()),
        )?,
        management_api_client_secret: secret_store.get("MANAGEMENT_API_CLIENT_SECRET").ok_or(
            error::Error::NotFoundSecrets("MANAGEMENT_API_CLIENT_SECRET".into()),
        )?,
        management_api_audience: secret_store.get("MANAGEMENT_API_AUDIENCE").ok_or(
            error::Error::NotFoundSecrets("MANAGEMENT_API_AUDIENCE".into()),
        )?,
        jwks_url: secret_store
            .get("JWKS_URL")
            .ok_or(error::Error::NotFoundSecrets("JWKS_URL".into()))?,
    };

    key_init(&auth_secret).await?;

    let db_connector = ids_database::init(pool).await?;

    let db = State::new(db_connector, secret_store);

    Ok(db)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize, Debug, Clone)]
    pub struct Config {
        access_token_url: String,
        management_api_client_id: String,
        management_api_client_secret: String,
        management_api_audience: String,
        jwks_url: String,
        database_url: String,
    }

    pub struct TestState {
        pub db: Arc<DbConnector>,
        pub secret_store: Config,
    }

    impl TestState {
        pub fn new(db: DbConnector, secret_store: Config) -> Self {
            Self {
                db: Arc::new(db),
                secret_store,
            }
        }

        pub fn db(&self) -> Arc<DbConnector> {
            self.db.clone()
        }
    }

    pub async fn init_util(config: Config) -> Result<TestState, Error> {
        let pool = PgPool::connect(&config.database_url).await.unwrap();

        let auth_secret = KeyInitConfig {
            access_token_url: config.access_token_url.clone(),
            management_api_client_id: config.management_api_client_id.clone(),
            management_api_client_secret: config.management_api_client_secret.clone(),
            management_api_audience: config.management_api_audience.clone(),
            jwks_url: config.jwks_url.clone(),
        };

        key_init(&auth_secret).await?;

        let db_connector = ids_database::init(pool).await?;

        let db = TestState::new(db_connector, config.clone());

        Ok(db)
    }

    pub async fn test_util() -> TestState {
        dotenvy::dotenv().ok();

        let config = envy::from_env::<Config>().unwrap();
        let test_state = init_util(config).await.unwrap();

        test_state
    }
}
