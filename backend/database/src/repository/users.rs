use std::sync::Arc;

use crate::error::Error;
use crate::DbConnector;

use derive_new::new;

#[derive(Debug, new)]
pub struct CreateUserEntity {
    pub id: String,
    pub user_email: String,
    pub picture: String,
}

#[derive(Debug, new)]
pub struct InputUserEntity {
    pub id: String,
    pub user_email: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub picture: String,
}

pub struct UserRepository(Arc<DbConnector>);

impl UserRepository {
    pub fn new(db: Arc<DbConnector>) -> Self {
        Self(db)
    }

    pub async fn create(&self, input: CreateUserEntity) -> Result<(), Error> {
        let pool = self.0.get_pool();

        let res = sqlx::query!(
            r#"
                INSERT INTO users
                    (id, user_email, given_name, family_name, picture)
                VALUES
                    ($1, $2, $3, $4, $5)
                    ON CONFLICT DO NOTHING
            "#,
            input.id,
            input.user_email,
            input.picture,
        )
        .execute(&pool)
        .await
        .map_err(Error::DatabaseError)?;

        if res.rows_affected() == 0 {
            return Err(Error::AlreadyExsited("user".into()));
        }
        Ok(())
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
