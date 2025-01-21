use crate::error::Error;
use crate::model::auth_user::AuthUser;
use crate::{model::request_bodies::classes::RequestPostClass, State};

use ids_database::{ClassesRepository, InputClassEntity};
use ids_shared::ClassId;
use tracing::{span, Level};

use axum::{
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use axum_macros::debug_handler;
use std::sync::Arc;

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_handler))
        .route("/", post(post_handler))
}
/// クラス一覧取得
#[debug_handler]
pub async fn get_handler(
    auth_user: Extension<AuthUser>,
    state: Extension<Arc<State>>,
) -> Result<impl IntoResponse, Error> {
    let span = span!(Level::INFO, "api/v1/classes", method = "GET");
    let _enter = span.enter();

    let repo = ClassesRepository::new(state.db.clone());

    let classes = repo.find_all(auth_user.id.clone()).await?;

    Ok(Json(classes))
}

/// クラス作成
#[debug_handler]
pub async fn post_handler(
    auth_user: Extension<AuthUser>,
    state: Extension<Arc<State>>,
    Json(body): Json<RequestPostClass>,
) -> Result<impl IntoResponse, Error> {
    let span = span!(Level::INFO, "api/v1/classes", method = "POST");
    let _enter = span.enter();

    let repo = ClassesRepository::new(state.db.clone());

    let class = repo
        .create(InputClassEntity::new(
            ClassId::new_v4(),
            auth_user.id.clone(),
            body.class_name,
            body.age,
        ))
        .await?;

    Ok(Json(class))
}
