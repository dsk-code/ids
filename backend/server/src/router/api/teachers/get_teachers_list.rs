use crate::error::Error;
use crate::model::auth_user::AuthUserExt;
use crate::query::Paginations;
use crate::State;

use ids_database::{PostgresTeachersRepository, TeachersRepository};

use axum::extract::Query;
use axum::http::StatusCode;
use axum::{response::IntoResponse, Extension, Json};
use std::sync::Arc;
use tracing::{span, Level};

pub async fn get_teachers_list<A: AuthUserExt>(
    Query(paginations): Query<Paginations>,
    auth_user: Extension<A>,
    state: Extension<Arc<State>>,
) -> Result<impl IntoResponse, Error> {
    let span = span!(Level::INFO, "api/v1/teachers", method = "GET");
    let _enter = span.enter();

    let repo = PostgresTeachersRepository::new(state.db.clone());

    let teachers = repo
        .find_all_with_pagination(auth_user.id(), paginations.into())
        .await?;

    Ok((StatusCode::OK, Json(teachers)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::router::api::teachers::post_teachers::post_teachers;
    use crate::router::api::test_utils::{test_unauthorization_middleware, TestInit, TestUser};

    use axum::body::{to_bytes, Body};
    use axum::http::{self, Request, StatusCode};
    use axum::middleware::from_fn;
    use axum::{routing::post, Router};
    use chrono::NaiveDate;
    use fake::{Fake, Faker};
    use ids_database::PaginatedTeachersListEntity;
    use serde_json::json;
    use tower::ServiceExt;

    #[rstest::rstest]
    #[case(Faker.fake::<String>(), 45, 10, 5, 50)]
    #[tokio::test]
    async fn test_get_teachers_list(
        #[case] token: String,
        #[case] offset: i32,
        #[case] limit: i32,
        #[case] expected_length: usize,
        #[case] total: i64,
    ) {
        let init = TestInit::new().await;

        let payload = json!({
            "lastName": (1..=50).fake::<String>(),
            "firstName": (1..=50).fake::<String>(),
            "lastNameKana": Some("カタカナ".to_string()),
            "firstNameKana": Some("カタカナ".to_string()),
            "phone": (1..=15).fake::<Option<String>>(),
            "mobilePhone": (1..=15).fake::<Option<String>>(),
            "email": (1..=255).fake::<Option<String>>(),
            "postCode1": "666".to_string(),
            "postCode2": "6666".to_string(),
            "prefecture": (1..=30).fake::<String>(),
            "city": (1..=40).fake::<String>(),
            "streetAddress": (1..=100).fake::<String>(),
            "building": (1..=50).fake::<Option<String>>(),
            "prefecturesKana": Some("カタカナ".to_string()),
            "cityKana": Some("カタカナ".to_string()),
            "buildingKana": Some("カタカナ".to_string()),
            "hireDate": Faker.fake::<NaiveDate>()
        });

        let test_router: Router = Router::new()
            .route(
                "/",
                post(post_teachers::<TestUser>).get(get_teachers_list::<TestUser>),
            )
            .layer(from_fn(test_unauthorization_middleware))
            .layer(Extension(init.get_state()));

        for _i in 1..=50 {
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

        let res = test_router
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri(format!("/?offset={}&limit={}", offset, limit))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header(http::header::AUTHORIZATION, format!("Bearer {}", token))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let body_bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let result: PaginatedTeachersListEntity = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(result.teachers.len(), expected_length);
        assert_eq!(result.total, Some(total));
    }
}
