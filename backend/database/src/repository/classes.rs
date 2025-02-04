use ids_shared as shared;

use crate::error::Error;
use crate::DbConnector;

use shared::{ClassId, UserId};

use async_trait::async_trait;
use chrono::NaiveDateTime;
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, span, Level};

#[derive(Debug, new, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClassEntity {
    pub id: ClassId,
    pub user_id: UserId,
    pub class_name: String,
    pub age: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, new, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InputClassEntity {
    pub id: ClassId,
    pub user_id: UserId,
    pub class_name: String,
    pub age: i32,
}

#[derive(Debug, new)]
pub struct InputFindClassEntity {
    pub id: ClassId,
    pub user_id: UserId,
}

#[derive(Debug, new, Clone)]
pub struct InputUpdateClassEntity {
    pub id: ClassId,
    pub user_id: UserId,
    pub class_name: String,
    pub age: i32,
}

#[derive(Debug, new)]
pub struct InputDeleteClassEntity {
    pub id: ClassId,
    pub user_id: UserId,
}

#[async_trait]
pub trait ClassesRepository {
    async fn create(&self, input: InputClassEntity) -> Result<ClassEntity, Error>;
    async fn find_all(&self, input: UserId) -> Result<Vec<ClassEntity>, Error>;
    async fn find_class(&self, input: InputFindClassEntity) -> Result<ClassEntity, Error>;
    async fn update(&self, input: InputUpdateClassEntity) -> Result<ClassEntity, Error>;
    async fn delete(&self, input: InputDeleteClassEntity) -> Result<(), Error>;
}

#[derive(Clone)]
pub struct PostgresClassesRepository(Arc<DbConnector>);

impl PostgresClassesRepository {
    pub fn new(db: Arc<DbConnector>) -> Self {
        Self(db)
    }
}

#[async_trait]
impl ClassesRepository for PostgresClassesRepository {
    /// クラスの作成
    async fn create(&self, input: InputClassEntity) -> Result<ClassEntity, Error> {
        let span = span!(Level::INFO, "db_query_execution", function = "create");
        let _enter = span.enter();

        let pool = self.0.get_pool();

        let user_id = input.user_id.id();

        info!("New class registration started");
        let class = sqlx::query_as!(
            ClassEntity,
            r#"
                INSERT INTO classes
                    (id, user_id, class_name, age)
                VALUES
                    ($1, $2, $3, $4)
                RETURNING id, user_id, class_name, age, created_at, updated_at
            "#,
            input.id.id(),
            user_id,
            input.class_name,
            input.age
        )
        .fetch_one(&pool)
        .await
        .map_err(Error::DatabaseError)?;
        info!("Successful new class registration");

        Ok(class)
    }

    /// すべてのクラスを検索
    async fn find_all(&self, input: UserId) -> Result<Vec<ClassEntity>, Error> {
        let span = span!(
            Level::INFO,
            "db_query_execution",
            function = "find_all_class"
        );
        let _enter = span.enter();

        let pool = self.0.get_pool();

        info!("Start find_all ClassesEntity");
        let classes = sqlx::query_as!(
            ClassEntity,
            r#"
                SELECT id, user_id, class_name, age, created_at, updated_at
                FROM classes
                WHERE user_id = $1
                ORDER BY age ASC
            "#,
            input.id(),
        )
        .fetch_all(&pool)
        .await
        .map_err(Error::DatabaseError)?;
        info!("Successful search for ClassesEntity");

        Ok(classes)
    }

    /// クラスを検索
    async fn find_class(&self, input: InputFindClassEntity) -> Result<ClassEntity, Error> {
        let span = span!(Level::INFO, "db_query_execution", function = "find_class");
        let _enter = span.enter();

        let pool = self.0.get_pool();

        info!("Start find_class ClassesEntity");
        let class = sqlx::query_as!(
            ClassEntity,
            r#"
                SELECT id, user_id, class_name, age, created_at, updated_at
                FROM classes
                WHERE id = $1::UUID AND user_id = $2::UUID
            "#,
            input.id.id(),
            input.user_id.id()
        )
        .fetch_one(&pool)
        .await
        .map_err(Error::DatabaseError)?;
        info!("Successful search for ClassesEntity");

        Ok(class)
    }

    async fn update(&self, input: InputUpdateClassEntity) -> Result<ClassEntity, Error> {
        let span = span!(Level::INFO, "db_query_execution", function = "update");
        let _enter = span.enter();

        let pool = self.0.get_pool();

        info!("Start updating in the classes table");
        let res = sqlx::query_as!(
            ClassEntity,
            r#"
                UPDATE classes
                SET class_name = $1, age = $2
                WHERE user_id = $3::UUID AND id = $4::UUID
                RETURNING id, user_id, class_name, age, created_at, updated_at
            "#,
            input.class_name,
            input.age,
            input.user_id.id(),
            input.id.id()
        )
        .fetch_one(&pool)
        .await
        .map_err(Error::DatabaseError)?;
        info!("Successfully updated is_active in classes table");

        Ok(res)
    }

    async fn delete(&self, input: InputDeleteClassEntity) -> Result<(), Error> {
        let span = span!(Level::INFO, "db_query_execution", function = "delete");
        let _enter = span.enter();

        let pool = self.0.get_pool();

        info!("Start deleting the classes table");
        sqlx::query!(
            r#"
                DELETE FROM classes WHERE id = $1::UUID AND user_id = $2::UUID
            "#,
            input.id.id(),
            input.user_id.id(),
        )
        .execute(&pool)
        .await
        .map_err(Error::DatabaseError)?;
        info!("Successfully deleted classes table");

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::repository::{
        test_utils::test_db_connector, users::test_utils::test_util_create_user,
    };

    use fake::{Fake, Faker};
    use sqlx::{PgPool, Result};

    // https://docs.rs/sqlx/latest/sqlx/attr.test.html
    #[sqlx::test(migrations = "./migrations")]
    async fn test_create(pool: PgPool) -> Result<()> {
        // ready
        let db = test_db_connector(pool.clone());
        let repo = PostgresClassesRepository::new(Arc::new(db));

        let user = test_util_create_user(pool).await;

        let input = InputClassEntity::new(ClassId::new_v4(), user.id, Faker.fake(), Faker.fake());
        // test
        let res = repo.create(input).await;

        assert!(res.is_ok());

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_find_all(pool: PgPool) -> Result<()> {
        // ready
        let db = test_db_connector(pool.clone());
        let repo = PostgresClassesRepository::new(Arc::new(db));

        let user = test_util_create_user(pool).await;

        for _i in 1..=100 {
            let input = InputClassEntity::new(
                ClassId::new_v4(),
                user.id.clone(),
                Faker.fake(),
                Faker.fake(),
            );
            repo.create(input).await.unwrap();
        }

        // test
        let res = repo.find_all(user.id).await.unwrap();

        assert_eq!(res.len(), 100);

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_find_class(pool: PgPool) -> Result<()> {
        // ready
        let db = test_db_connector(pool.clone());
        let repo = PostgresClassesRepository::new(Arc::new(db));

        let user = test_util_create_user(pool).await;

        let expected = InputClassEntity::new(
            ClassId::new_v4(),
            user.id.clone(),
            Faker.fake(),
            Faker.fake(),
        );
        repo.create(expected.clone()).await.unwrap();

        for _i in 1..100 {
            let input = InputClassEntity::new(
                ClassId::new_v4(),
                user.id.clone(),
                Faker.fake(),
                Faker.fake(),
            );
            repo.create(input).await.unwrap();
        }

        // test
        let res = repo
            .find_class(InputFindClassEntity::new(expected.id.clone(), user.id))
            .await
            .unwrap();

        assert_eq!(res.id, expected.id);
        assert_eq!(res.user_id, expected.user_id);
        assert_eq!(res.class_name, expected.class_name);
        assert_eq!(res.age, expected.age);

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_update(pool: PgPool) -> Result<()> {
        // ready
        let db = test_db_connector(pool.clone());
        let repo = PostgresClassesRepository::new(Arc::new(db));

        let user = test_util_create_user(pool).await;

        let input = InputClassEntity::new(
            ClassId::new_v4(),
            user.id.clone(),
            Faker.fake(),
            Faker.fake(),
        );
        repo.create(input.clone()).await.unwrap();

        // test
        let expected = InputUpdateClassEntity::new(
            input.id.clone(),
            user.id.clone(),
            Faker.fake(),
            Faker.fake(),
        );
        repo.update(expected.clone()).await.unwrap();

        let res = repo
            .find_class(InputFindClassEntity::new(input.id, user.id))
            .await
            .unwrap();

        assert_eq!(expected.id, res.id);
        assert_eq!(expected.user_id, res.user_id);
        assert_eq!(expected.class_name, res.class_name);
        assert_eq!(expected.age, res.age);

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_delete(pool: PgPool) -> Result<()> {
        // ready
        let db = test_db_connector(pool.clone());
        let repo = PostgresClassesRepository::new(Arc::new(db));

        let user = test_util_create_user(pool).await;

        let input = InputClassEntity::new(
            ClassId::new_v4(),
            user.id.clone(),
            Faker.fake(),
            Faker.fake(),
        );
        repo.create(input.clone()).await.unwrap();

        // test
        let res = repo
            .delete(InputDeleteClassEntity::new(input.id, user.id))
            .await;

        assert!(res.is_ok());

        Ok(())
    }
}
