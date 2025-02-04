pub mod me;

use axum::{routing::post, Router};

use crate::model::auth_user::{AuthUser, AuthUserExt};

pub fn router<A: AuthUserExt>() -> Router {
    Router::new().route("/", post(me::handler::<AuthUser>))
}
