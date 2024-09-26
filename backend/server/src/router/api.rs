pub mod protected;
use crate::State;

use std::sync::Arc;
use axum::{Extension, Router};

pub fn api(state: Arc<State>) -> Router {
    let router = Router::new()
        // .nest_service("/protected", service)
        .layer(Extension(state));
    router
}