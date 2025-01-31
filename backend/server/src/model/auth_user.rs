use ids_database as db;
use ids_shared as shard;

// use auth::Claims;
use db::UserEntity;
use shard::{UserId, Claims};

#[derive(Debug, Clone)]
pub struct AuthUser<C: Claims> {
    pub id: UserId,
    pub name: Option<String>,
    pub claims: C,
}

impl<C: Claims> AuthUser<C> {
    pub fn id(&self) -> shard::UserId {
        self.id.clone()
    }
}

impl<C: Claims> From<(db::UserEntity, C)> for AuthUser<C> {
    fn from((user, claims): (UserEntity, C)) -> Self {
        Self {
            id: user.id,
            name: user.auth0_user_name,
            claims,
        }
    }
}


// use ids_auth as auth;
// use ids_database as db;
// use ids_shared as shard;

// use auth::Claims;
// use db::UserEntity;
// use shard::UserId;

// #[derive(Debug, Clone)]
// pub struct AuthUser {
//     pub id: UserId,
//     pub name: Option<String>,
//     pub claims: Claims,
// }

// impl AuthUser {
//     pub fn id(&self) -> shard::UserId {
//         self.id.clone()
//     }
// }

// impl From<(db::UserEntity, auth::Claims)> for AuthUser {
//     fn from((user, claims): (UserEntity, auth::Claims)) -> Self {
//         Self {
//             id: user.id,
//             name: user.auth0_user_name,
//             claims,
//         }
//     }
// }
