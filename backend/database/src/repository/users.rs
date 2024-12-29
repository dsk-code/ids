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
    pub auth0_user_name: Option<String>,
    pub auth0_user_email: Option<String>,
}

#[derive(Debug, new, Clone)]
pub struct InputUserEntity {
    pub auth0_id: Auth0Id,
    pub auth0_user_name: Option<String>,
    pub auth0_user_email: Option<String>,
}

#[derive(Clone)]
pub struct UserRepository(Arc<DbConnector>);

impl UserRepository {
    pub fn new(db: Arc<DbConnector>) -> Self {
        Self(db)
    }

    pub async fn create(&self, input: InputUserEntity) -> Result<(), Error> {
        let pool = self.0.get_pool();

        println!("New user auth0_id registration started");
        let res = sqlx::query!(
            r#"
                INSERT INTO users
                    (auth0_id)
                VALUES
                    ($1)
                ON CONFLICT DO NOTHING
            "#,
            input.auth0_id.id(),
        )
        .execute(&pool)
        .await
        .map_err(Error::DatabaseError)?;

        // 重複していないかの確認
        if res.rows_affected() == 0 {
            return Err(Error::AlreadyExsited("users".into()));
        }
        println!("Successful new user auth0_id registration");

        Ok(())
    }

    pub async fn find_by_id(&self, id: Auth0Id) -> Result<UserEntity, Error> {
        let pool = self.0.get_pool();

        println!("Start find UserEntity");
        let user = sqlx::query_as!(
            UserEntity,
            r#"
                SELECT id, auth0_user_name, auth0_user_email
                FROM users
                WHERE auth0_id = $1
            "#,
            id.id()
        )
        .fetch_one(&pool)
        .await
        .map_err(|e| Error::DatabaseError(e))?;
        println!("Successful search for UserEntity");

        Ok(user)
    }

    pub async fn update(&self, input: InputUserEntity) -> Result<(), Error> {
        let pool = self.0.get_pool();

        println!("Start updating names and emails in the users table");
        sqlx::query!(
            r#"
                UPDATE users
                SET auth0_user_name = $1, auth0_user_email = $2
                WHERE auth0_id = $3
            "#,
            input.auth0_user_name,
            input.auth0_user_email,
            input.auth0_id.id(),
        )
        .execute(&pool)
        .await
        .map_err(|e| Error::DatabaseError(e))?;
        println!("Successfully updated name and email in users table");

        Ok(())
    }

    pub async fn delete(&self, id: Auth0Id) -> Result<(), Error> {
        let pool = self.0.get_pool();

        println!("Start deleting the users table");
        sqlx::query!(
            r#"
                DELETE FROM users WHERE auth0_id = $1
            "#,
            id.id()
        )
        .execute(&pool)
        .await
        .map_err(|e| Error::DatabaseError(e))?;
        println!("Successfully deleted users table");

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use crate::tests::util_init;

    use super::*;

    pub async fn users_create_test(auth0_id: Auth0Id) {
        let repo = UserRepository(util_init().await.unwrap());
        let auth0_user_name = "test".to_string();
        let auth0_user_email = "test@test.com".to_string();

        let user_entity =
            InputUserEntity::new(auth0_id, Some(auth0_user_name), Some(auth0_user_email));
        let user = repo.create(user_entity).await;

        assert!(user.is_ok());
    }

    async fn users_find_test(auth0_id: Auth0Id, expected: Option<String>) {
        let repo = UserRepository(util_init().await.unwrap());
        let user = repo.find_by_id(auth0_id).await.unwrap();

        assert_eq!(expected, user.auth0_user_name);
    }

    pub async fn users_find_id(auth0_id: Auth0Id) -> UserEntity {
        let repo = UserRepository(util_init().await.unwrap());
        let user = repo.find_by_id(auth0_id).await.unwrap();

        user
    }

    async fn users_update_test(auth0_id: Auth0Id) {
        let update_name = "test2".to_string();
        let update_email = "test2@test2.com".to_string();
        let repo = UserRepository(util_init().await.unwrap());
        let input = InputUserEntity::new(auth0_id, Some(update_name.clone()), Some(update_email));

        let user = repo.update(input.clone()).await;

        assert!(user.is_ok());
    }

    pub async fn users_delete_test(auth0_id: Auth0Id) {
        let repo = UserRepository(util_init().await.unwrap());
        let user = repo.delete(auth0_id).await;

        assert!(user.is_ok());
    }

    // CRUDの一連のテスト
    #[tokio::test]
    async fn users_test_in_order() {
        let auth0_id = Auth0Id::from("test".to_string());
        let create_name = "test".to_string();
        let update_name = "test2".to_string();

        users_create_test(auth0_id.clone()).await;
        users_find_test(auth0_id.clone(), Some(create_name)).await;
        users_update_test(auth0_id.clone()).await;
        users_find_test(auth0_id.clone(), Some(update_name)).await;
        users_delete_test(auth0_id.clone()).await;
    }
}
