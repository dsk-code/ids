use crate::error::Error;
use crate::model::responses::teachers::ResponseTeacherId;
use crate::model::{auth_user::AuthUserExt, request_bodies::teachers::RequestPostTeacher};
use crate::State;

use ids_database::{InputTeacherEntity, PostgresTeachersRepository, TeachersRepository};
use ids_shared::types::id::TeacherId;

use axum::http::StatusCode;
use axum::{response::IntoResponse, Extension, Json};
use std::sync::Arc;
use tracing::{span, Level};

pub async fn post_teachers<A: AuthUserExt>(
    auth_user: Extension<A>,
    state: Extension<Arc<State>>,
    Json(body): Json<RequestPostTeacher>,
) -> Result<impl IntoResponse, Error> {
    let span = span!(Level::INFO, "api/v1/teachers", method = "POST");
    let _enter = span.enter();

    let repo = PostgresTeachersRepository::new(state.db.clone());

    let id = repo
        .create(InputTeacherEntity {
            id: TeacherId::new_v4(),
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

    Ok((StatusCode::CREATED, Json(ResponseTeacherId { id })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::router::api::test_utils::{test_unauthorization_middleware, TestInit, TestUser};

    use axum::body::Body;
    use axum::http::{self, Request, StatusCode};
    use axum::middleware::from_fn;
    use axum::{routing::post, Router};
    use chrono::NaiveDate;
    use fake::{Fake, Faker};
    use serde_json::json;
    use tower::ServiceExt;

    #[rstest::rstest]
    #[case(Faker.fake::<String>())]
    #[tokio::test]
    async fn test_create_teachers(#[case] token: String) {
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
            .route("/", post(post_teachers::<TestUser>))
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
