use ids_server as server;

use anyhow::Context as _;
use axum::{extract::Request, http::HeaderValue, response::Response, Router};
use shuttle_runtime::SecretStore;
use sqlx::types::Uuid;
use std::{sync::Arc, time::Duration};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{Level, Span};

#[shuttle_runtime::main]
async fn main(
    // #[shuttle_shared_db::Postgres(local_uri = "{secrets.DATABASE_URL}")] pool: sqlx::PgPool,
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let state = server::init(secrets.clone(), pool)
        .await
        .context("failed to init")?;
    let state = Arc::new(state);

    let api = server::router::api::api(state.clone()).layer(
        TraceLayer::new_for_http()
            .make_span_with(|_req: &Request<_>| {
                let request_id = Uuid::new_v4();
                tracing::span!(
                    Level::INFO,
                    "apis",
                    request_id = tracing::field::display(request_id)
                )
            })
            .on_request(|req: &Request<_>, _span: &Span| {
                tracing::info!("[Request Start]");
                tracing::info!(
                    "request: {{method: {}, uri: {}, version: {:?}, headers: {{host: {:?}, content-type: {:?}, content-length: {:?}}}}}",
                    req.method(),
                    req.uri(),
                    req.version(),
                    req.headers().get("host"),
                    req.headers().get("content-type"),
                    req.headers().get("content-length")
                );
            })
            .on_response(|res: &Response<_>, _latency: Duration, _span: &Span| {
                tracing::info!("[Request End]");
                tracing::info!("response: {res:?}");
            }),
    );

    let origins = [
        secrets
            .get("CORS_URL_1")
            .expect("REASON")
            .parse::<HeaderValue>()
            .unwrap(),
        secrets
            .get("CORS_URL_2")
            .expect("REASON")
            .parse::<HeaderValue>()
            .unwrap(),
    ];

    let router = Router::new()
        .merge(server::router::static_file::static_roouter())
        .nest("/api/v1", api)
        .layer(CorsLayer::new().allow_origin(origins).allow_methods(Any));

    Ok(router.into())
}
