use ids_server as server;

use axum::{http::HeaderValue, Router};
use tower_http::cors::{Any, CorsLayer};
use shuttle_runtime::SecretStore;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(
        local_uri = "{secrets.DATABASE_URL}"
    )] pool: sqlx::PgPool,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let state = server::init(secrets, pool)
        .await
        .context("failed to init")?;
    let state = Arc::new(state);

    let origins = [
        secrets.get("CORS_URL_1").parse::<HeaderValue>().unwrap(),
        secrets.get("CORS_URL_2").parse::<HeaderValue>().unwrap(),
    ];

    let router = Router::new()
        .merge(server::router::static_file::static_roouter())
        .layer(CorsLayer::new()
            .allow_origin(origins)
            .allow_methods(Any)
        );

    Ok(router.into())
}