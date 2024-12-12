use ids_shared as shared;

use crate::error::Error;
use crate::DbConnector;

use derive_new::new;
use shared::UserId;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, new)]
pub struct UserEntity {
    pub id: UserId,
    pub user_name: Option<String>,
}

#[derive(Debug, new)]
pub struct CreateUserEntity {
    pub id: Uuid,
    pub user_name: String,
    pub user_email: String,
    pub picture: String,
    pub auth0_id: String,
}

#[derive(Debug, new)]
pub struct InputUserEntity {
    pub id: String,
    pub user_email: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub picture: String,
}

#[derive(Debug, new)]
pub struct InputUserValidateEntity {
    pub auth0_id: String,
    pub email: String,
}

pub struct UserRepository(Arc<DbConnector>);

impl UserRepository {
    pub fn new(db: Arc<DbConnector>) -> Self {
        Self(db)
    }

    pub async fn create(&self, input: CreateUserEntity) -> Result<(), Error> {
        let pool = self.0.get_pool();
        let mut tx = pool.begin().await?;
        println!("transaction start");

        println!("Start inserting into users table");
        let user_id = sqlx::query_scalar!(
            r#"
                INSERT INTO users
                    (id, user_name, user_email, picture)
                VALUES
                    ($1, $2, $3, $4)
                ON CONFLICT DO NOTHING
                RETURNING id
            "#,
            input.id,
            input.user_name,
            input.user_email,
            input.picture,
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(Error::DatabaseError)?;

        let user_id = match user_id {
            Some(id) => {
                println!("Successfully inserted into users table");
                id
            }
            None => return Err(Error::AlreadyExsited("user".into())),
        };

        println!("Start inserting into user_credentials table");
        sqlx::query!(
            r#"
            INSERT INTO user_credentials (user_id, auth0_id)
            VALUES ($1, $2)
            "#,
            user_id,
            input.auth0_id
        )
        .execute(&mut *tx)
        .await?;
        println!("Successfully inserted into user_credentials table");

        tx.commit().await?;
        Ok(())
    }

    pub async fn validate_get(&self, input: InputUserValidateEntity) -> Result<UserEntity, Error> {
        let pool = self.0.get_pool();

        let user = sqlx::query_as!(
            UserEntity,
            r#"
                SELECT u.id, u.user_name
                FROM user_credentials uc
                JOIN users u ON uc.user_id = u.id
                WHERE uc.auth0_id = $1
                AND u.user_email = $2
            "#,
            input.auth0_id,
            input.email,
        )
        .fetch_one(&pool)
        .await
        .map_err(|e| Error::DatabaseError(e))?;

        Ok(user)
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
