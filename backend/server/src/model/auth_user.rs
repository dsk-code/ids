use ids_auth as auth;
use ids_database as db;
use ids_shared as shard;

use auth::Claims;
use db::UserEntity;
use shard::UserId;

pub trait AuthUserExt: Send + Sync + 'static {
    fn id(&self) -> UserId;
    fn sub(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: UserId,
    pub name: Option<String>,
    pub claims: Claims,
}

impl From<(db::UserEntity, auth::Claims)> for AuthUser {
    fn from((user, claims): (UserEntity, auth::Claims)) -> Self {
        Self {
            id: user.id,
            name: user.auth0_user_name,
            claims,
        }
    }
}

impl AuthUserExt for AuthUser {
    fn id(&self) -> UserId {
        self.id.clone()
    }
    fn sub(&self) -> String {
        self.claims.sub()
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
