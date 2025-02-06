use crate::error::Error;
use crate::model::auth_user::AuthUserExt;
use crate::model::request_bodies::teachers::RequestPutTeacher;
use crate::model::responses::teachers::ResponseTeacherId;
use crate::State;

use ids_database::{InputTeacherEntity, PostgresTeachersRepository, TeachersRepository};
use ids_shared::types::id::TeacherId;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::{response::IntoResponse, Extension, Json};
use std::sync::Arc;
use tracing::{span, Level};
use uuid::Uuid;

pub async fn put_teacher<A: AuthUserExt>(
    auth_user: Extension<A>,
    state: Extension<Arc<State>>,
    Path(teacher_id): Path<Uuid>,
    Json(body): Json<RequestPutTeacher>,
) -> Result<impl IntoResponse, Error> {
    let span = span!(Level::INFO, "api/v1/teachers/:teacher_id", method = "PUT");
    let _enter = span.enter();

    let repo = PostgresTeachersRepository::new(state.db.clone());

    let id = repo
        .update(InputTeacherEntity {
            id: TeacherId::from(teacher_id),
            user_id: auth_user.id(),
            last_name: body.last_name,
            first_name: body.first_name,
            last_name_kana: body.last_name_kana,
            first_name_kana: body.first_name_kana,
            phone: body.phone,
            mobile_phone: body.mobile_phone,
            email: body.email,
            post_code1: body.post_code1,
            post_code2: body.post_code2,
            prefecture: body.prefecture,
            city: body.city,
            street_address: body.street_address,
            building: body.building,
            prefectures_kana: body.prefectures_kana,
            city_kana: body.city_kana,
            building_kana: body.building_kana,
            hire_date: body.hire_date,
        })
        .await?;

    Ok((StatusCode::OK, Json(ResponseTeacherId { id })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::responses::teachers::ResponseTeacherId;
    use crate::router::api::teachers::get_teacher::get_teacher;
    use crate::router::api::teachers::post_teachers::post_teachers;
    use crate::router::api::test_utils::{test_unauthorization_middleware, TestInit, TestUser};

    use axum::body::{to_bytes, Body};
    use axum::http::{self, Request, StatusCode};
    use axum::middleware::from_fn;
    use axum::routing::get;
    use axum::{routing::post, Router};
    use chrono::NaiveDate;
    use fake::{Fake, Faker};
    use ids_database::TeacherEntity;
    use serde_json::json;
    use tower::ServiceExt;

    #[rstest::rstest]
    #[case(Faker.fake::<String>(), "test")]
    #[tokio::test]
    async fn test_put_teacher(#[case] token: String, #[case] expected_last_name: &str) {
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

        let update_payload = json!({
            "lastName": expected_last_name,
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
            .route("/", post(post_teachers::<TestUser>))
            .route(
                "/{teacher_id}",
                get(get_teacher::<TestUser>).put(put_teacher::<TestUser>),
            )
            .layer(from_fn(test_unauthorization_middleware))
            .layer(Extension(init.get_state()));

        let get_res = test_router
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

        let body_bytes = to_bytes(get_res.into_body(), usize::MAX).await.unwrap();
        let teacher_id: ResponseTeacherId = serde_json::from_slice(&body_bytes).unwrap();

        let update_res = test_router
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/{}", teacher_id.id.clone().id()))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header(http::header::AUTHORIZATION, format!("Bearer {}", token))
                    .body(Body::from(serde_json::to_string(&update_payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(update_res.status(), StatusCode::OK);

        let result = test_router
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri(format!("/{}", teacher_id.id.id()))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header(http::header::AUTHORIZATION, format!("Bearer {}", token))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(result.status(), StatusCode::OK);

        let body_bytes = to_bytes(result.into_body(), usize::MAX).await.unwrap();
        let teacher: TeacherEntity = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(teacher.last_name, expected_last_name.to_string());
    }
}
