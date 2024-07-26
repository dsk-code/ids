use axum::{http::HeaderValue, Router};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::cors::{Any, CorsLayer};


// async fn hello_world() -> &'static str {
//     "Hello, world!"
// }

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let origins = [
        "http://localhost:3000".parse::<HeaderValue>().unwrap(),
        "https://kids-mng.vercel.app".parse::<HeaderValue>().unwrap(),
    ];

    let router = Router::new()
        .merge(static_roouter())
        .layer(CorsLayer::new()
            .allow_origin(origins)
            .allow_methods(Any)
        );

    Ok(router.into())
}

pub fn static_roouter() -> Router {
    Router::new()
        .nest_service("/assets", ServeDir::new("frontend/dist/assets"))
        .fallback_service(
            ServeDir::new("frontend/dist").not_found_service(ServeFile::new("frontend/dist/index.html")),
        )
}
// .nest_service("/assets", ServeDir::new("assets"))
//         .fallback_service(
//             ServeDir::new("dist").not_found_service(ServeFile::new("dist/index.html")),