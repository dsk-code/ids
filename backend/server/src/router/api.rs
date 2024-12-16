pub mod root;

use crate::middleware::authorization::authorization_middleware;
use crate::State;

use axum::middleware::from_fn_with_state;
use axum::{Extension, Router};
use std::sync::Arc;

pub fn api(state: Arc<State>) -> Router {
    let router = Router::new()
        .nest_service("/me", root::router())
        // stateをミドルウェア関数に渡す
        .layer(from_fn_with_state(state.clone(), authorization_middleware))
        .layer(Extension(state));
    router
}
