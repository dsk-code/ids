use ids_auth as auth;
// use ids_database as db;
// use ids_shared as shard;

use auth::Claims;
// use shard::UserId;

#[derive(Debug, Clone)]
pub struct AuthUser {
    // pub id: UserId,
    // pub name: Option<String>,
    pub claims: Claims,
}

// impl AuthUser {
//     pub fn id(&self) -> shard::UserId {
//         self.id.clone()
//     }
// }

impl From<auth::Claims> for AuthUser {
    fn from(claims: auth::Claims) -> Self {
        Self { claims: claims }
    }
}
