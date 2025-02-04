use crate::error::Error;
use crate::model::auth_user::{AuthUser, AuthUserExt};
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
// use axum_macros::debug_handler;
use std::sync::Arc;
use uuid::Uuid;

pub fn router<A: AuthUserExt>() -> Router {
    Router::new()
        .route(
            "/",
            post(post_class::<AuthUser>).get(get_classes_list::<AuthUser>),
        )
        .route(
            "/{class_id}",
            put(update_class::<AuthUser>)
                .get(get_class::<AuthUser>)
                .delete(delete_class::<AuthUser>),
        )
}

/// クラス一覧取得
/// debug_handlerはジェネリックをサポートしていない
// #[debug_handler]
pub async fn get_classes_list<A: AuthUserExt>(
    auth_user: Extension<A>,
    state: Extension<Arc<State>>,
) -> Result<impl IntoResponse, Error> {
    let span = span!(Level::INFO, "api/v1/classes", method = "GET");
    let _enter = span.enter();

    let repo = ClassesRepository::new(state.db.clone());

    let classes = repo.find_all(auth_user.id()).await?;

    Ok((StatusCode::OK, (Json(classes))))
}

/// クラス作成
// #[debug_handler]
pub async fn post_class<A: AuthUserExt>(
    auth_user: Extension<A>,
    state: Extension<Arc<State>>,
    Json(body): Json<RequestPostClass>,
) -> Result<impl IntoResponse, Error> {
    let span = span!(Level::INFO, "api/v1/classes", method = "POST");
    let _enter = span.enter();

    let repo = ClassesRepository::new(state.db.clone());

    let class = repo
        .create(InputClassEntity::new(
            ClassId::new_v4(),
            auth_user.id(),
            body.class_name,
            body.age,
        ))
        .await?;

    Ok((StatusCode::CREATED, Json(class)))
}

/// クラス更新
// #[debug_handler]
pub async fn update_class<A: AuthUserExt>(
    auth_user: Extension<A>,
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
            auth_user.id(),
            body.class_name,
            body.age,
        ))
        .await?;

    Ok((StatusCode::OK, Json(class)))
}

/// クラス検索
// #[debug_handler]
pub async fn get_class<A: AuthUserExt>(
    auth_user: Extension<A>,
    state: Extension<Arc<State>>,
    Path(class_id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    let span = span!(Level::INFO, "api/v1/classes/:class_id", method = "GET");
    let _enter = span.enter();

    let repo = ClassesRepository::new(state.db.clone());

    let class = repo
        .find_class(InputFindClassEntity::new(
            ClassId::from(class_id),
            auth_user.id(),
        ))
        .await?;

    Ok((StatusCode::OK, Json(class)))
}

/// クラス削除
// #[debug_handler]
pub async fn delete_class<A: AuthUserExt>(
    auth_user: Extension<A>,
    state: Extension<Arc<State>>,
    Path(class_id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    let span = span!(Level::INFO, "api/v1/classes/:class_id", method = "DELETE");
    let _enter = span.enter();

    let repo = ClassesRepository::new(state.db.clone());

    let _class = repo
        .delete(InputDeleteClassEntity::new(
            ClassId::from(class_id),
            auth_user.id(),
        ))
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

// note: 参考
// #[cfg(test)]
// pub mod test_utils {
//     use super::*;

//     use crate::{ws::ChatRooms, Config};
//     // use axum::{extract::Request, middleware::Next, response::Response};
//     use rust_study_auth::JWT;
//     use rust_study_db_connector::DbConnector;
//     use rust_study_auth as auth;

//     use anyhow::Error;
//     // use rust_study_shared::UserId;
//     use sqlx::PgPool;
//     use tokio::sync::Mutex;
//     use uuid::Uuid;

//     pub struct TestUserData {
//         id: Uuid,
//         name: String,
//         email: String,
//         password: String,
//     }

//     impl TestUserData {
//         pub fn new() -> Self {
//             let user = TestUserData {
//                 id: Uuid::parse_str("3FF7B560-9836-4A84-811D-4C7FAD8CF921").unwrap(),
//                 name: "dummy".to_string(),
//                 email: "dummydummydummy@dummydummy.com".to_string(),
//                 password: "dummy".to_string(),
//             };

//             user
//         }
//     }

//     pub async fn test_util_init(config: Config) -> Arc<State> {
//         auth::init(config.auth.secret_key.as_bytes()).unwrap();

//         let pool = PgPool::connect(&config.db.url).await.unwrap();
//         let db = Arc::new(DbConnector::new(pool, config.db.secret_key));
//         let chat_rooms = Arc::new(Mutex::new(ChatRooms::default()));

//         Arc::new(State { db, chat_rooms })
//     }

//     // pub fn test_util_claims() -> Claims {
//     //     let user = TestUserData::new();

//     //     let jwt = JWT::create(
//     //         "http::/localhost:8080/api/v1".to_string(),
//     //         user.id,
//     //         "http::/localhost:8080/api/v1".to_string(),
//     //         48,
//     //     )
//     //     .unwrap();

//     //     let claims = jwt.validate("http::/localhost:8080/api/v1").unwrap();

//     //     claims
//     // }

//     pub fn test_util_access_token() -> JWT {
//         let user = TestUserData::new();

//         let jwt = JWT::create(
//             "http::/localhost:8080/api/v1".to_string(),
//             user.id,
//             "http::/localhost:8080/api/v1".to_string(),
//             48,
//         )
//         .unwrap();

//         jwt
//     }

//     pub async fn test_util_create_user(pool: PgPool, db_secret: String) -> Result<(), Error> {
//         let user = TestUserData::new();

//         let _res = sqlx::query!(
//             r#"
//                 INSERT INTO users
//                     (id, name, email, password)
//                 values
//                     ($1::UUID, $2, digest($3, 'sha256'), pgp_sym_encrypt_bytea($4, $5))
//                     ON CONFLICT DO NOTHING
//             "#,
//             user.id,
//             user.name,
//             user.email,
//             user.password.as_bytes(),
//             db_secret,
//         )
//         .execute(&pool)
//         .await
//         .unwrap();

//         Ok(())
//     }

//     pub async fn test_util_delete(pool: PgPool) -> Result<(), Error> {
//         let user = TestUserData::new();

//         let _res = sqlx::query!(
//             r#"
//                 DELETE FROM users WHERE id = $1
//             "#,
//             user.id,
//         )
//         .execute(&pool)
//         .await
//         .unwrap();

//         Ok(())
//     }

//     // pub async fn test_authorization_middleware(
//     //     request: Request,
//     //     next: Next,
//     // ) -> Response {
//     //     let user = TestUserData::new();

//     //     let (mut parts, body) = request.into_parts();

//     //     let auth_user = AuthUser {
//     //         id: UserId::from(user.id),
//     //         name: Some(user.name),
//     //         claims: test_util_claims()
//     //     };

//     //     parts.extensions.insert(auth_user.clone());
//     //     let request = Request::from_parts(parts, body);

//     //     next.run(request).await
//     // }

// }

// #[cfg(test)]
// mod tests {
//     use crate::middleware::authorization_middleware;
//     use crate::Config;

//     use super::*;

//     use axum::body::Body;
//     use axum::middleware::from_fn_with_state;
//     use test_utils::{test_util_access_token, test_util_create_user, test_util_delete, test_util_init};

//     use std::fs;
//     use axum::{routing::post, Router};
//     use axum::http::{self, Request, StatusCode};
//     use tower::ServiceExt;
//     use serde_json::json;

//     #[tokio::test]
//     async fn test_create_planet() {
//         let config_file = "./../../../.development/config.toml";
//         let contents = fs::read_to_string(config_file).unwrap();
//         let config: Config = toml::from_str(&contents).unwrap();

//         let state = test_util_init(config.clone()).await;

//         test_util_create_user(state.db.get_pool(), config.db.secret_key).await.unwrap();

//         let token = test_util_access_token();

//         let payload = json!({
//             "name": "test",
//             "description": "test"
//         });
//         let test_router: Router = Router::new()
//             .route("/", post(handler))
//             .layer(from_fn_with_state(state.clone(), authorization_middleware))
//             // .layer(middleware::from_fn(test_authorization_middleware))
//             .layer(Extension(state));

//         let res = test_router.oneshot(
//             Request::builder()
//                .method(http::Method::POST)
//                .uri("/")
//                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
//                .header(http::header::AUTHORIZATION, format!("Bearer {}", token.access_token()))
//                .body(Body::from(serde_json::to_string(&payload).unwrap()))
//                .unwrap()
//             )
//             .await
//             .unwrap();

//         assert_eq!(res.status(), StatusCode::OK);
//     }

//     #[tokio::test]
//     async fn test_delete_planet() {
//         let config_file = "./../../../.development/config.toml";
//         let contents = fs::read_to_string(config_file).unwrap();
//         let config: Config = toml::from_str(&contents).unwrap();

//         let state = test_util_init(config.clone()).await;

//         let s  = test_util_delete(state.db.get_pool()).await;

//         assert!(s.is_ok());
//     }

// }
