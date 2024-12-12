pub mod protected;
use crate::State;

use axum::{Extension, Router};
use std::sync::Arc;

pub fn api(state: Arc<State>) -> Router {
    let router = Router::new()
        // .nest_service("/protected", service)
        .layer(Extension(state));
    router
}
