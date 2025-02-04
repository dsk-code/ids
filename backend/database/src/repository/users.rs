use ids_shared as shared;

use crate::error::Error;
use crate::DbConnector;

use async_trait::async_trait;
use derive_new::new;
use serde::{Deserialize, Serialize};
use shared::{Auth0Id, UserId};
use std::sync::Arc;
use tracing::{info, span, Level};

#[derive(Debug, new, Deserialize, Serialize)]
pub struct UserEntity {
    pub id: UserId,
    auth0_id: Auth0Id,
    pub auth0_user_name: Option<String>,
    pub auth0_user_email: Option<String>,
}

#[derive(Debug, new, Clone, PartialEq)]
pub struct InputUserEntity {
    pub auth0_id: Auth0Id,
}

#[derive(Debug, new, Clone, PartialEq)]
pub struct InputUpdateUserEntity {
    pub auth0_id: Auth0Id,
    pub auth0_user_name: Option<String>,
    pub auth0_user_email: Option<String>,
}

#[async_trait]
pub trait UserRepository {
    async fn create(&self, input: InputUserEntity) -> Result<(), Error>;
    async fn find_by_id(&self, id: Auth0Id) -> Result<UserEntity, Error>;
    async fn update(&self, input: InputUpdateUserEntity) -> Result<(), Error>;
    async fn delete(&self, id: Auth0Id) -> Result<(), Error>;
}

#[derive(Clone)]
pub struct PostgresUserRepository(Arc<DbConnector>);

impl PostgresUserRepository {
    pub fn new(db: Arc<DbConnector>) -> Self {
        Self(db)
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, input: InputUserEntity) -> Result<(), Error> {
        let span = span!(Level::INFO, "db_query_execution", function = "create");
        let _enter = span.enter();

        let pool = self.0.get_pool();

        info!("New user auth0_id registration started");
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
        info!("Successful new user auth0_id registration");

        Ok(())
    }

    async fn find_by_id(&self, id: Auth0Id) -> Result<UserEntity, Error> {
        let span = span!(Level::INFO, "db_query_execution", function = "find_by_id");
        let _enter = span.enter();

        let pool = self.0.get_pool();

        info!("Start find UserEntity");
        let user = sqlx::query_as!(
            UserEntity,
            r#"
                SELECT id, auth0_id, auth0_user_name, auth0_user_email
                FROM users
                WHERE auth0_id = $1
            "#,
            id.id()
        )
        .fetch_one(&pool)
        .await
        .map_err(Error::DatabaseError)?;
        info!("Successful search for UserEntity");

        Ok(user)
    }

    async fn update(&self, input: InputUpdateUserEntity) -> Result<(), Error> {
        let span = span!(Level::INFO, "db_query_execution", function = "update");
        let _enter = span.enter();

        let pool = self.0.get_pool();

        info!("Start updating names and emails in the users table");
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
        .map_err(Error::DatabaseError)?;
        info!("Successfully updated name and email in users table");

        Ok(())
    }

    async fn delete(&self, id: Auth0Id) -> Result<(), Error> {
        let span = span!(Level::INFO, "db_query_execution", function = "delete");
        let _enter = span.enter();

        let pool = self.0.get_pool();

        info!("Start deleting the users table");
        sqlx::query!(
            r#"
                DELETE FROM users WHERE auth0_id = $1
            "#,
            id.id()
        )
        .execute(&pool)
        .await
        .map_err(Error::DatabaseError)?;
        info!("Successfully deleted users table");

        Ok(())
    }
}

#[cfg(test)]
pub mod test_utils {
    use fake::{Fake, Faker};
    use sqlx::PgPool;

    use crate::repository::test_utils::test_db_connector;

    use super::*;

    pub async fn test_util_create_user(pool: PgPool) -> UserEntity {
        let db = test_db_connector(pool);
        let repo = PostgresUserRepository::new(Arc::new(db));

        let auth0_id = Auth0Id::from(Faker.fake::<String>());

        let input = InputUserEntity::new(auth0_id.clone());
        repo.create(input).await.unwrap();

        repo.find_by_id(auth0_id.clone()).await.unwrap()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::repository::test_utils::test_db_connector;

    use super::*;

    use fake::{
        faker::{internet::raw::FreeEmail, name::raw::Name},
        locales::EN,
        Fake, Faker,
    };
    use sqlx::{PgPool, Result};

    // https://docs.rs/sqlx/latest/sqlx/attr.test.html
    #[sqlx::test(migrations = "./migrations")]
    async fn test_create(pool: PgPool) -> Result<()> {
        // ready
        let db = test_db_connector(pool);
        let repo = PostgresUserRepository::new(Arc::new(db));

        let input = InputUserEntity::new(Auth0Id::from(Faker.fake::<String>()));

        // test
        let res = repo.create(input).await;

        assert!(res.is_ok());

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_find_by_id(pool: PgPool) -> Result<()> {
        // ready
        let db = test_db_connector(pool);
        let repo = PostgresUserRepository::new(Arc::new(db));

        let expected = InputUserEntity::new(Auth0Id::from(Faker.fake::<String>()));
        repo.create(expected.clone()).await.unwrap();

        for _i in 1..=100 {
            let input = InputUserEntity::new(Auth0Id::from(Faker.fake::<String>()));
            repo.create(input).await.unwrap();
        }

        // test
        let res = repo.find_by_id(expected.auth0_id.clone()).await.unwrap();

        assert_eq!(res.auth0_id, expected.auth0_id);

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_update(pool: PgPool) -> Result<()> {
        // ready
        let db = test_db_connector(pool);
        let repo = PostgresUserRepository::new(Arc::new(db));

        let auth0_id = Auth0Id::from(Faker.fake::<String>());

        let input = InputUserEntity::new(auth0_id.clone());
        repo.create(input).await.unwrap();

        let expected = InputUpdateUserEntity::new(
            auth0_id.clone(),
            Some(Name(EN).fake()),
            Some(FreeEmail(EN).fake()),
        );
        repo.update(expected.clone()).await.unwrap();

        // test
        let res = repo.find_by_id(auth0_id.clone()).await.unwrap();

        assert_eq!(res.auth0_id, auth0_id);
        assert_eq!(res.auth0_user_name, expected.auth0_user_name);
        assert_eq!(res.auth0_user_email, expected.auth0_user_email);

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_delete(pool: PgPool) -> Result<()> {
        // ready
        let db = test_db_connector(pool);
        let repo = PostgresUserRepository::new(Arc::new(db));

        let auth0_id = Auth0Id::from(Faker.fake::<String>());

        let input = InputUserEntity::new(auth0_id.clone());
        repo.create(input).await.unwrap();

        // test
        let res = repo.delete(auth0_id.clone()).await;

        assert!(res.is_ok());

        Ok(())
    }
}
