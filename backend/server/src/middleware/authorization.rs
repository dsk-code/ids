use ids_auth as auth;
use ids_database as db;

use crate::error::Error;
use crate::model::auth_user::AuthUser;
use crate::State;

use db::{InputUserEntity, UserRepository};

// use axum::async_trait;
use axum::extract::{FromRef, FromRequestParts, Request};
use axum::http::request::Parts;
use axum::{middleware::Next, response::Response, Extension, RequestPartsExt};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use std::sync::Arc;
use tracing::{info, span, Level};

pub async fn authorization_middleware(
    state: Extension<Arc<State>>,
    request: Request,
    next: Next, // 次のミドルウェアまたはハンドラ
) -> Result<Response, Error> {
    let span = span!(Level::INFO, "authorization_middleware");
    let _enter = span.enter();

    info!("Starting middleware");
    let (mut parts, body) = request.into_parts();
    // 認証ユーザーの抽出
    let auth_user = AuthUser::from_request_parts(&mut parts, &state).await?;
    info!("auth_userが抽出されました");

    // リクエストの再構築
    let mut request = Request::from_parts(parts, body);
    // 抽出した auth_user をリクエストの extensions に追加
    request.extensions_mut().insert(auth_user.clone());

    // ミドルウェアの次の段階に処理を渡す
    info!("Middleware termination");
    Ok(next.run(request).await)
}

// #[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    Arc<State>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // state から共有状態を取得
        let app_state = Arc::<State>::from_ref(state);

        // リクエストヘッダーからBearerトークンを抽出
        info!("Start extracting tokens");
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                Error::RequiredAuthorization(e.to_string())
            })?;
        info!("Successful token extraction");

        // JWT の検証
        info!("Start validating the token");
        let jwt = auth::JWT::new(bearer.token().to_owned());
        let claims = jwt
            .validate(&ids_auth::ValidateConfig::new(
                app_state.secrets.aud.clone(),
                app_state.secrets.aud2.clone(),
                app_state.secrets.iss.clone(),
            ))
            .map_err(|e| {
                eprintln!("{}", e);
                Error::AuthError(e)
            })?;
        info!("Token validation successfully");

        // 在籍確認
        info!("Start checking user enrollment");
        let db = app_state.db();
        let repo = db::PostgresUserRepository::new(db);

        let user = repo.find_by_id(claims.sub().into()).await;
        let user = match user {
            Ok(user) => {
                info!("Successful user registration verification");
                return Ok(AuthUser::from((user, claims)));
            }
            Err(_) => {
                // ユーザーが見つからなかった場合、登録を行う
                info!("User not registered");
                info!("Start user registration");
                let create_user = InputUserEntity::new(claims.sub().into());
                repo.create(create_user).await?;
                // 登録後に再度ユーザーを検索
                info!("Start reconfirming user enrollment");
                repo.find_by_id(claims.sub().into()).await?
            }
        };

        info!("Successful user registration verification");
        Ok(AuthUser::from((user, claims)))
    }
}
