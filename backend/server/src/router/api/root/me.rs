use ids_database as db;

use crate::error::Error;
use crate::model::auth_user::AuthUserExt;
use crate::model::{request_bodies::me::RequestAuthUser, responses::me::ResponseAuthUser};
use crate::State;

use db::{InputUpdateUserEntity, UserRepository};

use axum::{response::IntoResponse, Extension, Json};
use std::sync::Arc;
use tracing::{info, span, Level};

pub async fn handler<A: AuthUserExt>(
    auth_user: Extension<A>,
    state: Extension<Arc<State>>,
    Json(body): Json<RequestAuthUser>,
) -> Result<impl IntoResponse, Error> {
    let span = span!(Level::INFO, "api/v1/me", method = "POST");
    let _enter = span.enter();

    let repo = db::PostgresUserRepository::new(state.db());

    // ユーザー名とメールの登録状況の確認
    info!("Check username and email registration status");
    let user = repo.find_by_id(auth_user.sub().into()).await?;

    if user.auth0_user_name == body.auth0_user_name
        && user.auth0_user_email == body.auth0_user_email
    {
        // ユーザー名とメールの登録状況の確認成功
        info!("Successful verification of username and email registration status");
        Ok(Json(ResponseAuthUser::from(user)))
    } else {
        // ユーザー名とメールの登録がされていない
        info!("Username and email not registered");
        let update_user = InputUpdateUserEntity::new(
            auth_user.sub().into(),
            body.auth0_user_name,
            body.auth0_user_email,
        );
        repo.update(update_user).await?;
        info!("Reconfirm username and email registration status");
        let user = repo.find_by_id(auth_user.sub().into()).await?;

        info!("Successful verification of username and email registration status");
        Ok(Json(ResponseAuthUser::from(user)))
    }
}
