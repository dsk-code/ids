use ids_database as db;

use crate::error::Error;
use crate::model::{
    auth_user::AuthUser, request_body::RequestAuthUser, response::ResponseAuthUser,
};
use crate::State;
use db::InputUserEntity;

use axum::{response::IntoResponse, Extension, Json};
use std::sync::Arc;

pub async fn handler(
    auth_user: Extension<AuthUser>,
    state: Extension<Arc<State>>,
    Json(body): Json<RequestAuthUser>,
) -> Result<impl IntoResponse, Error> {
    let repo = db::UserRepository::new(state.db());

    // ユーザー名とメールの登録状況の確認
    println!("Check username and email registration status");
    let user = repo.find_by_id(auth_user.claims.sub.clone().into()).await?;
    let user = match (user.auth0_user_name.clone(), user.auth0_user_email.clone()) {
        (Some(_), Some(_)) => {
            // ユーザー名とメールの登録状況の確認成功
            println!("Successful verification of username and email registration status");
            return Ok(Json(ResponseAuthUser::from(user)));
        }
        _ => {
            // ユーザー名とメールの登録がされていない
            println!("Username and email not registered");
            let update_user = InputUserEntity::new(
                auth_user.claims.sub.clone().into(),
                body.auth0_user_name,
                body.auth0_user_email,
            );
            repo.update(update_user).await?;
            println!("Reconfirm username and email registration status");
            repo.find_by_id(auth_user.claims.sub.clone().into()).await?
        }
    };

    println!("Successful verification of username and email registration status");
    Ok(Json(ResponseAuthUser::from(user)))
}
