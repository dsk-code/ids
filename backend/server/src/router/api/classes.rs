use crate::error::Error;
use crate::model::auth_user::AuthUser;
use crate::model::request_bodies::classes::RequestUpdateClass;
use crate::{model::request_bodies::classes::RequestPostClass, State};

use axum::extract::Path;
use axum::http::StatusCode;
use ids_database::{
    ClassesRepository, InputClassEntity, InputDeleteClassEntity, InputFindClassEntity,
    InputUpdateClassEntity,
};
use ids_shared::ClassId;
use tracing::{span, Level};

use axum::{
    response::IntoResponse,
    routing::{post, put},
    Extension, Json, Router,
};
use axum_macros::debug_handler;
use uuid::Uuid;
use std::sync::Arc;

pub fn router() -> Router {
    Router::new()
        .route("/", post(post_class).get(get_classes_list))
        .route(
            "/{class_id}",
            put(update_class).get(get_class).delete(delete_class),
        )
}

/// クラス一覧取得
#[debug_handler]
pub async fn get_classes_list(
    auth_user: Extension<AuthUser>,
    state: Extension<Arc<State>>,
) -> Result<impl IntoResponse, Error> {
    let span = span!(Level::INFO, "api/v1/classes", method = "GET");
    let _enter = span.enter();

    let repo = ClassesRepository::new(state.db.clone());

    let classes = repo.find_all(auth_user.id.clone()).await?;

    Ok((StatusCode::OK, (Json(classes))))
}

/// クラス作成
#[debug_handler]
pub async fn post_class(
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

    Ok((StatusCode::CREATED, Json(class)))
}

/// クラス更新
#[debug_handler]
pub async fn update_class(
    auth_user: Extension<AuthUser>,
    state: Extension<Arc<State>>,
    Path(class_id): Path<Uuid>,
    Json(body): Json<RequestUpdateClass>,
) -> Result<impl IntoResponse, Error> {
    let span = span!(Level::INFO, "api/v1/classes/:class_id", method = "PUT");
    let _enter = span.enter();

    let repo = ClassesRepository::new(state.db.clone());

    let class = repo
        .update(InputUpdateClassEntity::new(
            ClassId::from(class_id),
            auth_user.id.clone(),
            body.class_name,
            body.age,
        ))
        .await?;

    Ok((StatusCode::OK, Json(class)))
}

/// クラス検索
#[debug_handler]
pub async fn get_class(
    auth_user: Extension<AuthUser>,
    state: Extension<Arc<State>>,
    Path(class_id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    let span = span!(Level::INFO, "api/v1/classes/:class_id", method = "GET");
    let _enter = span.enter();

    let repo = ClassesRepository::new(state.db.clone());

    let class = repo
        .find_class(InputFindClassEntity::new(ClassId::from(class_id), auth_user.id.clone()))
        .await?;

    Ok((StatusCode::OK, Json(class)))
}

/// クラス削除
#[debug_handler]
pub async fn delete_class(
    auth_user: Extension<AuthUser>,
    state: Extension<Arc<State>>,
    Path(class_id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    let span = span!(Level::INFO, "api/v1/classes/:class_id", method = "DELETE");
    let _enter = span.enter();

    let repo = ClassesRepository::new(state.db.clone());

    let _class = repo
        .delete(InputDeleteClassEntity::new(ClassId::from(class_id), auth_user.id.clone()))
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
