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
