use crate::error::Error;
use crate::model::auth_user::AuthUserExt;
use crate::State;

use ids_database::repository::classes::PostgresClassesRepository;
use ids_database::ClassesRepository;

use axum::http::StatusCode;
use axum::{response::IntoResponse, Extension, Json};
use std::sync::Arc;
use tracing::{span, Level};

/// クラス一覧取得
pub async fn get_classes_list<A: AuthUserExt>(
    auth_user: Extension<A>,
    state: Extension<Arc<State>>,
) -> Result<impl IntoResponse, Error> {
    let span = span!(Level::INFO, "api/v1/classes", method = "GET");
    let _enter = span.enter();

    let repo = PostgresClassesRepository::new(state.db.clone());

    let classes = repo.find_all(auth_user.id()).await?;

    Ok((StatusCode::OK, (Json(classes))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::router::api::classes::post_classes::post_class;
    use crate::router::api::test_utils::{test_unauthorization_middleware, TestInit, TestUser};

    use axum::body::{to_bytes, Body};
    use axum::http::{self, Request, StatusCode};
    use axum::middleware::from_fn;
    use axum::{routing::post, Router};
    use fake::{Fake, Faker};
    use ids_database::ClassEntity;
    use serde_json::json;
    use std::usize;
    use tower::ServiceExt;

    #[rstest::rstest]
    #[case(Faker.fake::<String>())]
    #[tokio::test]
    async fn test_get_classes(#[case] token: String) {
        // ready
        let init = TestInit::new().await;

        let test_router: Router = Router::new()
            .route(
                "/",
                post(post_class::<TestUser>).get(get_classes_list::<TestUser>),
            )
            .layer(from_fn(test_unauthorization_middleware))
            .layer(Extension(init.get_state()));

        for _i in 1..=10 {
            let payload = json!({
            "className": Faker.fake::<String>(),
            "age": Faker.fake::<i32>()
            });

            test_router
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
        }

        // test
        let res = test_router
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header(http::header::AUTHORIZATION, format!("Bearer {}", token))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let body_bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let classes: Vec<ClassEntity> = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(classes.len(), 10);
    }
}
