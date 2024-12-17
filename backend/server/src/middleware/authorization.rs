use ids_auth as auth;
// use ids_database as db;

use crate::error::Error;
use crate::model::auth_user::AuthUser;
use crate::State;

use axum::async_trait;
use axum::extract::{FromRef, FromRequestParts, Request};
use axum::http::request::Parts;
use axum::{middleware::Next, response::Response, Extension, RequestPartsExt};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use std::sync::Arc;

pub async fn authorization_middleware(
    state: Extension<Arc<State>>,
    request: Request,
    next: Next, // 次のミドルウェアまたはハンドラ
) -> Result<Response, Error> {
    println!("リクエストが来ました");
    let (mut parts, body) = request.into_parts();
    // 認証ユーザーの抽出
    let auth_user = AuthUser::from_request_parts(&mut parts, &state).await?;
    println!("auth_userが抽出されました");
    // リクエストの再構築
    let mut request = Request::from_parts(parts, body);
    // 抽出した auth_user をリクエストの extensions に追加
    request.extensions_mut().insert(auth_user.clone());

    // ミドルウェアの次の段階に処理を渡す
    Ok(next.run(request).await)
}

#[async_trait]
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
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                Error::RequiredAuthorization(e.to_string())
            })?;

        // JWT の検証
        let jwt = auth::JWT::new(bearer.token().to_owned());
        let claims = jwt
            .validate(&ids_auth::ValidateConfig::new(
                app_state
                    .secret_store
                    .get("AUD")
                    .ok_or(Error::NotFoundSecrets("AUD".into()))?,
                app_state
                    .secret_store
                    .get("AUD2")
                    .ok_or(Error::NotFoundSecrets("AUD2".into()))?,
                app_state
                    .secret_store
                    .get("ISS")
                    .ok_or(Error::NotFoundSecrets("ISS".into()))?,
            ))
            .map_err(|e| {
                eprintln!("{}", e);
                Error::AuthError(e)
            })?;

        // // ユーザーの存在確認
        // let db = app_state.db();
        // let repo = db::UserRepository::new(db);
        // // ユーザーを検索
        // let user = match repo.find_by_id(claims.sub.clone().into()).await {
        //     Ok(user) => {
        //         // ユーザーが見つかった場合、そのまま返す
        //         return Ok(user);
        //     }
        //     Err(_) => {
        //         // ユーザーが見つからなかった場合、登録を行う
        //         repo.create(&claims.sub).await?;
        //         // 登録後に再度ユーザーを検索
        //         repo.find_by_id(claims.sub.clone().into()).await?
        //     }
        // };

        // let user = repo
        //     .find_by_id(claims.sub.clone().into())
        //     .await;
        // .map_err(|e| Error::RequiredAuthorization(e.to_string()))?;

        println!("認可されました");
        Ok(claims.into())
    }
}
