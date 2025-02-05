use crate::error::Error;
use crate::model::auth_user::AuthUserExt;
use crate::model::request_bodies::classes::RequestUpdateClass;
use crate::State;

use ids_database::repository::classes::PostgresClassesRepository;
use ids_database::{ClassesRepository, InputUpdateClassEntity};
use ids_shared::ClassId;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::{response::IntoResponse, Extension, Json};
use std::sync::Arc;
use tracing::{span, Level};
use uuid::Uuid;

/// クラス更新
pub async fn update_class<A: AuthUserExt>(
    auth_user: Extension<A>,
    state: Extension<Arc<State>>,
    Path(class_id): Path<Uuid>,
    Json(body): Json<RequestUpdateClass>,
) -> Result<impl IntoResponse, Error> {
    let span = span!(Level::INFO, "api/v1/classes/:class_id", method = "PUT");
    let _enter = span.enter();

    let repo = PostgresClassesRepository::new(state.db.clone());

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::router::api::classes::post_classes::post_class;
    use crate::router::api::test_utils::{test_unauthorization_middleware, TestInit, TestUser};

    use ids_database::ClassEntity;

    use axum::body::{to_bytes, Body};
    use axum::http::{self, Request, StatusCode};
    use axum::middleware::from_fn;
    use axum::{
        routing::{post, put},
        Router,
    };
    use fake::{Fake, Faker};
    use serde_json::json;
    use std::usize;
    use tower::ServiceExt;

    #[rstest::rstest]
    #[case(Faker.fake::<String>(), Faker.fake::<String>(), Faker.fake::<i32>())]
    #[tokio::test]
    async fn test_update_class(
        #[case] token: String,
        #[case] class_name: String,
        #[case] age: i32,
    ) {
        // ready
        let init = TestInit::new().await;

        let expected_name = "dummy";
        let expected_age = 10;

        let test_router: Router = Router::new()
            .route("/", post(post_class::<TestUser>))
            .route("/{class_id}", put(update_class::<TestUser>))
            .layer(from_fn(test_unauthorization_middleware))
            .layer(Extension(init.get_state()));

        let payload = json!({
            "className": class_name,
            "age": age
        });

        let expected_payload = json!({
            "className": expected_name,
            "age": expected_age
        });

        let res = test_router
            .clone()
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

        let body_bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let class: ClassEntity = serde_json::from_slice(&body_bytes).unwrap();

        // test
        let result = test_router
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/{}", class.id.id().to_string()))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header(http::header::AUTHORIZATION, format!("Bearer {}", token))
                    .body(Body::from(
                        serde_json::to_string(&expected_payload).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(result.status(), StatusCode::OK);

        let body_bytes = to_bytes(result.into_body(), usize::MAX).await.unwrap();
        let result_class: ClassEntity = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(result_class.class_name, expected_name.to_string());
        assert_eq!(result_class.age, expected_age);
    }
}
