use ids_database::UserEntity;
use ids_shared::UserId;

use serde::{Deserialize, Serialize};

/// POST api/vi/me
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseAuthUser {
    pub id: UserId,
    pub user_name: String,
}

impl From<UserEntity> for ResponseAuthUser {
    fn from(value: UserEntity) -> Self {
        Self {
            id: value.id,
            user_name: value.user_name,
        }
    }
}
