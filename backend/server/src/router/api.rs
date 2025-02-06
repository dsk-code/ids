pub mod classes;
pub mod root;
pub mod teachers;

use crate::middleware::authorization::authorization_middleware;
use crate::model::auth_user::{AuthUser, AuthUserExt};
use crate::State;

use axum::middleware::from_fn_with_state;
use axum::{Extension, Router};
use std::sync::Arc;
// todo: /classesを作成
pub fn api<A: AuthUserExt>(state: Arc<State>) -> Router {
    Router::new()
        .nest_service("/me", root::router::<AuthUser>())
        .nest_service("/classes", classes::router::<AuthUser>())
        // todo: teschers router
        // .nest_service("/teachers", service)
        .layer(from_fn_with_state(state.clone(), authorization_middleware))
        .layer(Extension(state))
}

#[cfg(test)]
pub mod test_utils {
    use super::*;
    use crate::error::Error;
    use crate::JWTValidationConfig;

    use db::{DbConnector, InputUserEntity, UserRepository};
    use shared::{Auth0Id, UserId};

    use axum::extract::{FromRef, FromRequestParts, Request};
    use axum::http::request::Parts;
    use axum::middleware::Next;
    use axum::response::Response;
    use axum::RequestPartsExt;
    use axum_extra::{
        headers::{authorization::Bearer, Authorization},
        TypedHeader,
    };
    // use fake::{Fake, Faker};
    use ids_database as db;
    use ids_shared as shared;

    #[derive(Clone)]
    pub struct TestInit {
        state: Arc<State>,
    }

    impl TestInit {
        pub async fn new() -> Self {
            let db_url = std::env::var("TEST_DATABASE_URL").unwrap();

            let db = Arc::new(db::init(db_url).await.unwrap());

            Self {
                state: Arc::new(State {
                    db,
                    secrets: JWTValidationConfig {
                        aud: "test".to_string(),
                        aud2: "test".to_string(),
                        iss: "test".to_string(),
                    },
                }),
            }
        }

        pub fn get_state(&self) -> Arc<State> {
            self.state.clone()
        }

        pub fn get_db(&self) -> Arc<DbConnector> {
            self.state.db.clone()
        }
    }

    #[derive(Debug, Clone)]
    pub struct TestUser {
        pub id: UserId,
        pub sub: String,
    }

    // impl TestUser {
    //     pub async fn new_with_create_user_db(db: Arc<DbConnector>) -> Self {
    //         let repo = db::PostgresUserRepository::new(db);
    //         let id = Auth0Id::from(Faker.fake::<String>());

    //         let input = InputUserEntity::new(id.clone());
    //         repo.create(input).await.unwrap();

    //         let res = repo.find_by_id(id.clone()).await.unwrap();

    //         Self {
    //             id: res.id,
    //             sub: id.id().to_string(),
    //         }
    //     }
    // }

    impl AuthUserExt for TestUser {
        fn id(&self) -> ids_shared::UserId {
            self.id.clone()
        }

        fn sub(&self) -> String {
            self.sub.clone()
        }
    }

    pub async fn test_unauthorization_middleware(
        state: Extension<Arc<State>>,
        request: Request,
        next: Next,
    ) -> Response {
        let (mut parts, body) = request.into_parts();

        let test_user = TestUser::from_request_parts(&mut parts, &state)
            .await
            .unwrap();

        let mut request = Request::from_parts(parts, body);
        request.extensions_mut().insert(test_user.clone());

        next.run(request).await
    }

    impl<S> FromRequestParts<S> for TestUser
    where
        Arc<State>: FromRef<S>,
        S: Send + Sync,
    {
        type Rejection = Error;

        async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
            // state から共有状態を取得
            let app_state = Arc::<State>::from_ref(state);

            // リクエストヘッダーからBearerトークンを抽出
            let TypedHeader(Authorization(bearer)) = parts
                .extract::<TypedHeader<Authorization<Bearer>>>()
                .await
                .unwrap();

            // 在籍確認
            let db = app_state.db();
            let repo = db::PostgresUserRepository::new(db);

            let user = repo
                .find_by_id(Auth0Id::from(bearer.token().to_string()))
                .await;
            let user = match user {
                Ok(user) => {
                    return Ok(TestUser {
                        id: user.id,
                        sub: bearer.token().to_string(),
                    });
                }
                Err(_) => {
                    // ユーザーが見つからなかった場合、登録を行う
                    let create_user =
                        InputUserEntity::new(Auth0Id::from(bearer.token().to_string()));
                    repo.create(create_user).await?;
                    // 登録後に再度ユーザーを検索
                    repo.find_by_id(Auth0Id::from(bearer.token().to_string()))
                        .await?
                }
            };
            Ok(TestUser {
                id: user.id,
                sub: bearer.token().to_string(),
            })
        }
    }
}
