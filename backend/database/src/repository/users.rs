use ids_shared as shared;

use crate::error::Error;
use crate::DbConnector;

use derive_new::new;
use serde::{Deserialize, Serialize};
use shared::{Auth0Id, UserId};
use std::sync::Arc;

#[derive(Debug, new, Deserialize, Serialize)]
pub struct UserEntity {
    pub id: UserId,
    pub user_name: String,
}

#[derive(Debug, new)]
pub struct CreateUserEntity {
    pub auth0_id: Auth0Id,
    pub user_name: String,
    pub user_email: String,
}

pub struct UserRepository(Arc<DbConnector>);

impl UserRepository {
    pub fn new(db: Arc<DbConnector>) -> Self {
        Self(db)
    }

    pub async fn create(&self, input: CreateUserEntity) -> Result<(), Error> {
        let pool = self.0.get_pool();

        println!("Start inserting into users table");
        let res = sqlx::query!(
            r#"
                INSERT INTO users
                    (auth0_id, user_name, user_email)
                VALUES
                    ($1, $2, $3)
                ON CONFLICT DO NOTHING
            "#,
            input.auth0_id.id(),
            input.user_name,
            input.user_email,
        )
        .execute(&pool)
        .await
        .map_err(Error::DatabaseError)?;
        println!("Successfully inserted into users table");

        // 重複していないかの確認
        if res.rows_affected() == 0 {
            return Err(Error::AlreadyExsited("users".into()));
        }

        Ok(())
    }

    pub async fn find_by_id(&self, id: Auth0Id) -> Result<UserEntity, Error> {
        let pool = self.0.get_pool();

        let user = sqlx::query_as!(
            UserEntity,
            r#"
                SELECT id, user_name
                FROM users
                WHERE auth0_id = $1
            "#,
            id.id()
        )
        .fetch_one(&pool)
        .await
        .map_err(|e| Error::DatabaseError(e))?;

        Ok(user)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::tests::util_init;

    use super::*;

    #[tokio::test]
    async fn user_create_test() {
        let repo = UserRepository(util_init().await.unwrap());
        let auth0_id = Auth0Id::from("test".to_string());
        let user_name = "test".to_string();

        let create_entity = CreateUserEntity {
            auth0_id: auth0_id.clone(),
            user_name: user_name.clone(),
            user_email: "test.@test.com".to_string(),
        };

        repo.create(create_entity).await.unwrap();

        let user_entity = repo.find_by_id(auth0_id).await.unwrap();

        assert_eq!(user_name, user_entity.user_name);



    }
}
// #[cfg(test)]
// pub mod tests_utils {
//     use super::*;
//     // use crate::repository::tests_utils::TEST_SECRET;
//     use sqlx::PgPool;
//     use fake::locales::EN;
//     use fake::Fake;
//     use fake::faker::name::raw::Name;
//     use fake::faker::internet::raw::{FreeEmail, Password};
//     use password_hash::{SaltString, rand_core::OsRng, PasswordHasher};
//     use argon2::Argon2;
//     use dotenv::{dotenv, var};

//     #[derive(Debug, new)]
//     pub struct TestUser {
//         pub id: String,
//         pub username: String,
//         pub email: String,
//         pub password_hash: String,
//     }

//     pub fn test_path() -> String {
//         dotenv().ok();
//         let path = var("DATABASE_URL").unwrap();
//         path
//     }

//     pub fn create_test_input_user_entity() -> Result<InputUserEntity, Error> {
//         let password_string = Password(EN, 15..20).fake::<String>();
//         let password = password_string.as_bytes();
//         let salt = SaltString::generate( &mut OsRng);
//         let argon2 = Argon2::default();
//         let password_hash = argon2.hash_password(password, &salt).unwrap()
//             .to_string();
//         let user = InputUserEntity {
//             username: Name(EN).fake::<String>(),
//             email: FreeEmail(EN).fake::<String>(),
//             password_hash,
//         };

//         Ok(user)
//     }

//     pub async fn create_test_user(pool: PgPool) -> Result<Uuid, Error> {
//         let input_user_entity = create_test_input_user_entity().unwrap();
//         let id = Uuid::new_v4();
//         let _ = sqlx::query!(
//             r#"
//                 INSERT INTO users
//                     (id, username, email, password_hash)
//                 VALUES
//                     ($1::UUID, $2, $3, $4)
//                     ON CONFLICT DO NOTHING
//             "#,
//             id,
//             input_user_entity.username,
//             input_user_entity.email,
//             input_user_entity.password_hash
//         )
//         .execute(&pool)
//         .await?;

//         Ok(id)
//     }

//     #[tokio::test]
//     async fn create_test() {
//         let path = test_path();
//         let pool = PgPool::connect(&path).await.unwrap();
//         let user = create_test_user(pool).await.unwrap();
//         println!("user = {}", user);
//     }

// }
