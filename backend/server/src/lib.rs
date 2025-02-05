// use ids_auth as auth;
use ids_database::DbConnector;

pub mod error;
pub mod middleware;
pub mod model;
pub mod router;

use ids_auth::{key_init, types::KeyInitConfig};

use error::Error;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub access_token_url: String,
    pub management_api_client_id: String,
    pub management_api_client_secret: String,
    pub management_api_audience: String,
    pub jwks_url: String,
    pub kid: String,
    pub aud: String,
    pub aud2: String,
    pub iss: String,
    pub cors_url_1: String,
    pub cors_url_2: String,
    pub port: String,
}

#[derive(Clone)]
pub struct JWTValidationConfig {
    pub aud: String,
    pub aud2: String,
    pub iss: String,
}

pub struct State {
    db: Arc<DbConnector>,
    secrets: JWTValidationConfig,
}

impl State {
    pub fn new(db: DbConnector, secrets: JWTValidationConfig) -> Self {
        Self {
            db: Arc::new(db),
            secrets,
        }
    }

    pub fn db(&self) -> Arc<DbConnector> {
        self.db.clone()
    }
}

pub async fn init(secrets: Config) -> Result<State, Error> {
    let auth_secret = KeyInitConfig {
        access_token_url: secrets.access_token_url,
        management_api_client_id: secrets.management_api_client_id,
        management_api_client_secret: secrets.management_api_client_secret,
        management_api_audience: secrets.management_api_audience,
        jwks_url: secrets.jwks_url,
    };

    key_init(&auth_secret).await?;

    let db_connector = ids_database::init(secrets.database_url.clone()).await?;

    let db = State::new(
        db_connector,
        JWTValidationConfig {
            aud: secrets.aud,
            aud2: secrets.aud2,
            iss: secrets.iss,
        },
    );

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
        let auth_secret = KeyInitConfig {
            access_token_url: config.access_token_url.clone(),
            management_api_client_id: config.management_api_client_id.clone(),
            management_api_client_secret: config.management_api_client_secret.clone(),
            management_api_audience: config.management_api_audience.clone(),
            jwks_url: config.jwks_url.clone(),
        };

        key_init(&auth_secret).await?;

        let db_connector = ids_database::init(config.database_url.clone()).await?;

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
