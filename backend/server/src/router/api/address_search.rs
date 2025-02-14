use std::time::Duration;

use axum::{extract::Query, http, response::IntoResponse, routing::get, Json, Router};
use reqwest::{Client, StatusCode};
use tracing::{span, Level};

use crate::{
    error::Error,
    model::{
        auth_user::{AuthUser, AuthUserExt},
        responses::address_search::ResponseGetAddress,
    },
    query::RequestGetAddress,
};

pub fn router<A: AuthUserExt>() -> Router {
    Router::new().route("/", get(get_address::<AuthUser>))
}

pub async fn get_address<A: AuthUserExt>(
    Query(get_address_query): Query<RequestGetAddress>,
) -> Result<impl IntoResponse, Error> {
    let span = span!(Level::INFO, "api/v1/address_search", method = "GET");
    let _enter = span.enter();

    let url = format!(
        "https://zipcloud.ibsnet.co.jp/api/search?zipcode={}",
        get_address_query.post_code
    );
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to build client");

    let response = client
        .get(&url)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .send()
        .await?
        .json::<ResponseGetAddress>()
        .await?;

    Ok((StatusCode::OK, Json(response)))
}

#[cfg(test)]
pub mod test {
    use axum::{
        body::{to_bytes, Body},
        extract::Request,
        Extension,
    };
    use tower::ServiceExt;

    use crate::router::api::test_utils::{TestInit, TestUser};

    use super::*;

    #[rstest::rstest]
    #[case("8390841", "福岡県", "久留米市", "御井旗崎")]
    #[tokio::test]
    async fn test_get_address(
        #[case] post_code: i32,
        #[case] address1: &str,
        #[case] address2: &str,
        #[case] address3: &str,
    ) {
        let init = TestInit::new().await;

        let test_router: Router = Router::new()
            .route("/", get(get_address::<TestUser>))
            .layer(Extension(init.get_state()));

        let res = test_router
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri(format!("/?postCode={}", post_code))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let body_bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let get_address: ResponseGetAddress = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(get_address.status, 200);
        for address in get_address.results {
            assert_eq!(address.address1, address1.to_string());
            assert_eq!(address.address2, address2.to_string());
            assert_eq!(address.address3, address3.to_string());
        }
    }
}
