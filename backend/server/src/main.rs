use ids_server::{self as server, error};

use crate::server::{error::Error, Config};

use anyhow::Context as _;
use axum::{extract::Request, http::HeaderValue, response::Response, Router};
use sqlx::types::Uuid;
use std::{sync::Arc, time::Duration};
use tower_http::{cors::Any, cors::CorsLayer, trace::TraceLayer};
use tracing::{Level, Span};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO) // INFO レベル以上のログを出力
        .init();

    dotenvy::dotenv().ok();
    let secrets = envy::from_env::<Config>()?;

    let state = server::init(secrets.clone())
        .await
        .context("failed to init")
        .map_err(|err| server::error::Error::InitError(err))?;
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
            .cors_url_1
            .parse::<HeaderValue>()
            .map_err(|err| error::Error::InvalidHeaderValue(err.to_string()))?,
        secrets
            .cors_url_2
            .parse::<HeaderValue>()
            .map_err(|err| error::Error::InvalidHeaderValue(err.to_string()))?,
    ];

    let router = Router::new()
        .merge(server::router::static_file::static_roouter())
        .nest("/api/v1", api)
        .layer(CorsLayer::new().allow_origin(origins).allow_methods(Any));

    let listener = tokio::net::TcpListener::bind(secrets.port)
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();

    Ok(())
}
