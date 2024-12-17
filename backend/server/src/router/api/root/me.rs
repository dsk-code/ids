use ids_database as db;

use crate::error::Error;
use crate::model::auth_user::AuthUser;
use crate::State;
use db::CreateUserEntity;

use axum::{response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    pub user_name: String,
    pub user_email: String,
}

pub async fn handler(
    auth_user: Extension<AuthUser>,
    state: Extension<Arc<State>>,
    Json(body): Json<RequestBody>,
) -> Result<impl IntoResponse, Error> {
    let repo = db::UserRepository::new(state.db());
    
    let user = repo.find_by_id(auth_user.claims.sub.clone().into()).await;
    let user = match user {
        Ok(user) => return Ok(Json(user)),
        Err(_) => {
            // ユーザーが見つからなかった場合、登録を行う
            let create_user = CreateUserEntity::new(
                auth_user.claims.sub.clone().into(),
                body.user_name,
                body.user_email,
            );
            repo.create(create_user).await?;
            // 登録後に再度ユーザーを検索
            repo.find_by_id(auth_user.claims.sub.clone().into()).await?
        }
    };

    Ok(Json(user))
}
