use ids_server as server;
use ids_server::router::api::api;

use axum::{http::HeaderValue, Router};
use tower_http::cors::{Any, CorsLayer};
use shuttle_runtime::SecretStore;
use std::sync::Arc;
use anyhow::Context as _;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(
        local_uri = "{secrets.DATABASE_URL}"
    )] pool: sqlx::PgPool,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let state = server::init(secrets.clone(), pool)
        .await
        .context("failed to init")?;
    let state = Arc::new(state);

    let origins = [
        secrets.get("CORS_URL_1").expect("REASON").parse::<HeaderValue>().unwrap(),
        secrets.get("CORS_URL_2").expect("REASON").parse::<HeaderValue>().unwrap(),
    ];

    let router = Router::new()
        .merge(server::router::static_file::static_roouter())
        .nest("/api/v1", api(state.clone()))
        .layer(CorsLayer::new()
            .allow_origin(origins)
            .allow_methods(Any)
        );

    Ok(router.into())
}