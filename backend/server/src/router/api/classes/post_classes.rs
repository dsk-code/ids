use crate::error::Error;
use crate::model::auth_user::AuthUserExt;
use crate::{model::request_bodies::classes::RequestPostClass, State};

use ids_database::repository::classes::PostgresClassesRepository;
use ids_database::{ClassesRepository, InputClassEntity};
use ids_shared::ClassId;

use axum::http::StatusCode;
use axum::{response::IntoResponse, Extension, Json};
use std::sync::Arc;
use tracing::{span, Level};

/// クラス作成
pub async fn post_class<A: AuthUserExt>(
    auth_user: Extension<A>,
    state: Extension<Arc<State>>,
    Json(body): Json<RequestPostClass>,
) -> Result<impl IntoResponse, Error> {
    let span = span!(Level::INFO, "api/v1/classes", method = "POST");
    let _enter = span.enter();

    let repo = PostgresClassesRepository::new(state.db.clone());

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::router::api::test_utils::{test_unauthorization_middleware, TestInit, TestUser};

    use axum::body::Body;
    use axum::http::{self, Request, StatusCode};
    use axum::middleware::from_fn;
    use axum::{routing::post, Router};
    use fake::{Fake, Faker};
    use serde_json::json;
    use tower::ServiceExt;

    #[rstest::rstest]
    #[case(Faker.fake::<String>(), Faker.fake::<String>(), Faker.fake::<i32>())]
    #[case(Faker.fake::<String>(), Faker.fake::<String>(), Faker.fake::<i32>())]
    #[case(Faker.fake::<String>(), Faker.fake::<String>(), Faker.fake::<i32>())]
    #[tokio::test]
    async fn test_create_classes(
        #[case] token: String,
        #[case] class_name: String,
        #[case] age: i32,
    ) {
        let init = TestInit::new().await;

        let payload = json!({
            "className": class_name,
            "age": age
        });

        let test_router: Router = Router::new()
            .route("/", post(post_class::<TestUser>))
            .layer(from_fn(test_unauthorization_middleware))
            .layer(Extension(init.get_state()));

        let res = test_router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header(http::header::AUTHORIZATION, format!("Bearer {}", token))
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::CREATED);
    }
}
