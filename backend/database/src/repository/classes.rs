use ids_shared as shared;

use crate::error::Error;
use crate::DbConnector;

use shared::{ClassId, UserId};

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

#[derive(Debug, new)]
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

#[derive(Clone)]
pub struct ClassesRepository(Arc<DbConnector>);

impl ClassesRepository {
    pub fn new(db: Arc<DbConnector>) -> Self {
        Self(db)
    }

    /// クラスの作成
    pub async fn create(&self, input: InputClassEntity) -> Result<ClassEntity, Error> {
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
    pub async fn find_all(&self, input: UserId) -> Result<Vec<ClassEntity>, Error> {
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
                "#,
            input.id(),
        )
        .fetch_all(&pool)
        .await
        .map_err(|e| Error::DatabaseError(e))?;
        info!("Successful search for ClassesEntity");

        Ok(classes)
    }

    /// クラスを検索
    pub async fn find_class(&self, input: InputFindClassEntity) -> Result<ClassEntity, Error> {
        let span = span!(Level::INFO, "db_query_execution", function = "find_class");
        let _enter = span.enter();

        let pool = self.0.get_pool();

        info!("Start find_class ClassesEntity");
        let class = sqlx::query_as!(
            ClassEntity,
            r#"
                    SELECT id, user_id, class_name, age, created_at, updated_at
                    FROM classes
                    WHERE id = $1 AND user_id = $2
                "#,
            input.id.id(),
            input.user_id.id()
        )
        .fetch_one(&pool)
        .await
        .map_err(|e| Error::DatabaseError(e))?;
        info!("Successful search for ClassesEntity");

        Ok(class)
    }

    pub async fn update(&self, input: InputUpdateClassEntity) -> Result<ClassEntity, Error> {
        let span = span!(Level::INFO, "db_query_execution", function = "update");
        let _enter = span.enter();

        let pool = self.0.get_pool();

        info!("Start updating in the classes table");
        let res = sqlx::query_as!(
            ClassEntity,
            r#"
                UPDATE classes
                SET class_name = $1, age = $2
                WHERE user_id = $3 AND id = $4
                RETURNING id, user_id, class_name, age, created_at, updated_at
            "#,
            input.class_name,
            input.age,
            input.user_id.id(),
            input.id.id()
        )
        .fetch_one(&pool)
        .await
        .map_err(|e| Error::DatabaseError(e))?;
        info!("Successfully updated is_active in classes table");

        Ok(res)
    }

    pub async fn delete(&self, input: InputDeleteClassEntity) -> Result<(), Error> {
        let span = span!(Level::INFO, "db_query_execution", function = "delete");
        let _enter = span.enter();

        let pool = self.0.get_pool();

        info!("Start deleting the classes table");
        sqlx::query!(
            r#"
                DELETE FROM classes WHERE id = $1 AND user_id = $2
            "#,
            input.id.id(),
            input.user_id.id(),
        )
        .execute(&pool)
        .await
        .map_err(|e| Error::DatabaseError(e))?;
        info!("Successfully deleted classes table");

        Ok(())
    }
}

// todo: テストを書く
#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::repository::users::InputUserEntity;
    use crate::{
        repository::users::tests::{users_create_test, users_delete_test, users_find_id},
        tests::util_init,
    };

    use ids_shared::Auth0Id;

    /// 作成テスト
    async fn classes_create_test(input: InputClassEntity) {
        let repo = ClassesRepository(util_init().await.unwrap());
        let class = repo.create(input.clone()).await.unwrap();

        assert_eq!(
            (input.user_id, input.class_name, input.age.clone()),
            (class.user_id, class.class_name, class.age)
        );
    }

    // 一覧検索の正常系テスト
    async fn classes_find_all(user_id: UserId, expected: Vec<TestClassData>) {
        let repo = ClassesRepository(util_init().await.unwrap());

        let classes = repo.find_all(user_id).await.unwrap();

        for l in 0..classes.len() {
            assert_eq!(
                (expected[l].name.clone(), expected[l].age),
                (classes[l].class_name.clone(), classes[l].age)
            );
        }
    }

    // クラス検索の正常系テスト
    async fn classes_find_class(input: InputFindClassEntity, expected: TestClassData) {
        let repo = ClassesRepository(util_init().await.unwrap());

        let class = repo.find_class(input).await.unwrap();

        assert_eq!(
            (expected.id, expected.name, expected.age),
            (class.id, class.class_name, class.age)
        );
    }

    // is_activeの更新、正常系テスト
    async fn classes_update(input: InputUpdateClassEntity, expected: TestClassData) {
        let repo = ClassesRepository(util_init().await.unwrap());

        let class = repo.update(input).await.unwrap();

        assert_eq!(
            (expected.id, expected.name, expected.age),
            (class.id, class.class_name, class.age)
        );
    }

    // レコード削除、正常系テスト
    async fn classes_delete(input: InputDeleteClassEntity) {
        let repo = ClassesRepository(util_init().await.unwrap());

        let res = repo.delete(input).await;

        assert!(res.is_ok());
    }

    #[derive(Debug, Clone, new, PartialEq)]
    pub struct TestUserDatas {
        user_data: InputUserEntity,
    }

    #[derive(Debug, Clone, new, PartialEq)]
    pub struct TestClassData {
        id: ClassId,
        name: String,
        age: i32,
    }

    /// classes reporitoryの正常系テスト
    #[tokio::test]
    async fn classes_test() {
        let auth0_id = Auth0Id::from("test".to_string());
        let auth0_user_name = "test".to_string();
        let auth0_user_email = "test@test.com".to_string();
        let datas = [
            (ClassId::new_v4(), "たまご", 0),
            (ClassId::new_v4(), "ひよこ", 1),
            (ClassId::new_v4(), "あひる", 2),
            (ClassId::new_v4(), "うさぎ", 3),
            (ClassId::new_v4(), "くま", 4),
            (ClassId::new_v4(), "ぞう", 5),
        ];

        let input_user = InputUserEntity::new(
            auth0_id.clone(),
            Some(auth0_user_name),
            Some(auth0_user_email),
        );

        let class_datas: Vec<TestClassData> = datas
            .clone()
            .into_iter()
            .map(|(id, name, age)| TestClassData::new(id, name.to_string(), age))
            .collect();

        let update_test_class =
            TestClassData::new(class_datas[2].id.clone(), "たけし".to_string(), 15);

        users_create_test(input_user).await;
        let user = users_find_id(auth0_id.clone()).await;

        let input_update = InputUpdateClassEntity::new(
            update_test_class.id.clone(),
            user.id.clone(),
            update_test_class.name.clone(),
            update_test_class.age.clone(),
        );

        let input_find = InputFindClassEntity::new(update_test_class.id.clone(), user.id.clone());

        let input_delete =
            InputDeleteClassEntity::new(update_test_class.id.clone(), user.id.clone());

        for class_data in class_datas.clone() {
            let input_class = InputClassEntity::new(
                class_data.id,
                user.id.clone(),
                class_data.name,
                class_data.age,
            );

            classes_create_test(input_class).await;
        }

        classes_find_all(user.id.clone(), class_datas.clone()).await;
        classes_update(input_update, update_test_class.clone()).await;
        classes_find_class(input_find, update_test_class.clone()).await;
        classes_delete(input_delete).await;
        users_delete_test(auth0_id.clone()).await;
    }
}
