use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

/// 静的ファイルのルーター reactなど
// pub fn static_roouter() -> Router {
//     Router::new()
//         .nest_service("/assets", ServeDir::new("frontend/dist/assets"))
//         .fallback_service(
//             ServeDir::new("frontend/dist")
//                 .not_found_service(ServeFile::new("frontend/dist/index.html")),
//         )
// }

/// Rimix用
pub fn static_roouter() -> Router {
    Router::new()
        .nest_service("/assets", ServeDir::new("frontend/build/client/assets"))
        .fallback_service(
            ServeDir::new("frontend/build/client")
                .not_found_service(ServeFile::new("frontend/build/client/index.html")),
        )
}
