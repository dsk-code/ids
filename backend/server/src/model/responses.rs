pub mod me;

// use ids_database::UserEntity;
// use ids_shared::UserId;

// use serde::{Deserialize, Serialize};

// /// POST api/vi/me
// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct ResponseAuthUser {
//     pub id: UserId,
//     pub auth0_user_name: Option<String>,
// }

// impl From<UserEntity> for ResponseAuthUser {
//     fn from(value: UserEntity) -> Self {
//         Self {
//             id: value.id,
//             auth0_user_name: value.auth0_user_name,
//         }
//     }
// }
