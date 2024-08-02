use auth;
use axum::http::header::AUTHORIZATION;
use axum::http::HeaderValue;
use db;

use crate::error::Error;
use argon2::Argon2;
use axum::response::{IntoResponse, Response};
use axum::{body::Body, routing::post, Extension, Json, Router};
use password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{error, State};

pub fn router(state: Arc<State>) -> Router {
    Router::new()
        .route("/signup", post(signup))
        // .route("login", post(login))
        .layer(Extension(state))
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SignupRequestBody {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub async fn signup(
    Extension(state): Extension<Arc<State>>,
    Json(body): Json<SignupRequestBody>,
) -> Result<impl IntoResponse, Error> {
    let repository = db::UserRepository::new(state.db());
    let password = body.password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password, &salt)
        .map_err(|_| error::Error::PasswordHashingError)?
        .to_string();
    let input = db::repository::users::InputUserEntity::new(body.name, body.email, password_hash);
    let id = Uuid::new_v4();
    repository.create(id, input).await?;
    let token = auth::JWT::create(
        "http::/localhost:8080/api/v1".to_string(),
        id,
        "http::/localhost:8080/api/v1".to_string(),
        48,
    )?
    .access_token()
    .to_owned();
    let response = Response::builder()
        .status(200)
        .header(
            AUTHORIZATION,
            format!("Bearer {}", token)
                .parse::<HeaderValue>()
                .map_err(|_| error::Error::NotFound("Converted token".to_string()))?,
        )
        .body(Body::from("User signed up successfully"))
        .map_err(|e| error::Error::NotFound(e.to_string()))?;

    Ok(response)
}
