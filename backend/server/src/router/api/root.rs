pub mod me;

use axum::{routing::post, Router};
use ids_auth::AuthClaims;
use ids_shared::Claims;

pub fn router<C: Claims>() -> Router {
    Router::new().route("/", post(me::handler::<AuthClaims>))
}
