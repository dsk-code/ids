pub mod classes;
pub mod root;

use crate::middleware::authorization::authorization_middleware;
use crate::model::auth_user::{AuthUser, AuthUserExt};
use crate::State;

use axum::middleware::from_fn_with_state;
use axum::{Extension, Router};
use std::sync::Arc;
// todo: /classesを作成
pub fn api<A: AuthUserExt>(state: Arc<State>) -> Router {
    let router = Router::new()
        .nest_service("/me", root::router::<AuthUser>())
        .nest_service("/classes", classes::router::<AuthUser>())
        // stateをミドルウェア関数に渡す
        .layer(from_fn_with_state(state.clone(), authorization_middleware))
        .layer(Extension(state));
    router
}
