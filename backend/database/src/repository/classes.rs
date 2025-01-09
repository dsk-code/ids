use ids_shared as shared;

use crate::error::Error;
use crate::DbConnector;

use shared::{ClassId, UserId};

use derive_new::new;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, span, Level};

#[derive(Debug, new, Deserialize, Serialize)]
pub struct ClassesEntity {
    pub id: ClassId,
    pub class_name: String,
    pub age: i32,
}

#[derive(Debug, new)]
pub struct InputClassEntity {
    pub user_id: UserId,
    pub class_name: String,
    pub age: i32,
}

#[derive(Debug, new)]
pub struct InputUpdateClassEntity {
    pub user_id: UserId,
    pub class_name: String,
    pub age: i32,
    pub is_active: bool,
}

#[derive(Debug, new)]
pub struct InputDeleteClassEntity {
    pub user_id: UserId,
    pub class_id: ClassId,
}

#[derive(Clone)]
pub struct ClassesRepository(Arc<DbConnector>);

impl ClassesRepository {
    pub fn new(db: Arc<DbConnector>) -> Self {
        Self(db)
    }

    /// クラスの作成（存在確認必要)
    pub async fn create(&self, input: InputClassEntity) -> Result<(), Error> {
        let span = span!(Level::INFO, "db_query_execution", function = "create");
        let _enter = span.enter();

        let pool = self.0.get_pool();

        let user_id = input.user_id.id();

        info!("New class registration started");
        let res = sqlx::query!(
            r#"
                INSERT INTO classes
                    (user_id, class_name, age)
                VALUES
                    ($1, $2, $3)
                ON CONFLICT DO NOTHING
            "#,
            user_id,
            input.class_name,
            input.age
        )
        .execute(&pool)
        .await
        .map_err(Error::DatabaseError)?;
        info!("Successful new class registration");

        // 重複していないかの確認
        if res.rows_affected() == 0 {
            return Err(Error::AlreadyExsited("classes".into()));
        }

        Ok(())
    }

    /// activeなクラスを検索
    pub async fn find_active_class(&self, input: UserId) -> Result<Vec<ClassesEntity>, Error> {
        let span = span!(
            Level::INFO,
            "db_query_execution",
            function = "find_active_class"
        );
        let _enter = span.enter();

        let pool = self.0.get_pool();

        info!("Start find ClassesEntity");
        let classes = sqlx::query_as!(
            ClassesEntity,
            r#"
                SELECT id, class_name, age
                FROM classes
                WHERE user_id = $1 AND is_active = true
            "#,
            input.id(),
        )
        .fetch_all(&pool)
        .await
        .map_err(|e| Error::DatabaseError(e))?;
        info!("Successful search for ClassesEntity");

        Ok(classes)
    }

    /// すべてのクラスを検索
    pub async fn find_all_class(&self, input: UserId) -> Result<Vec<ClassesEntity>, Error> {
        let span = span!(
            Level::INFO,
            "db_query_execution",
            function = "find_all_class"
        );
        let _enter = span.enter();

        let pool = self.0.get_pool();

        info!("Start find ClassesEntity");
        let classes = sqlx::query_as!(
            ClassesEntity,
            r#"
                    SELECT id, class_name, age
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

    /// class_nameの存在確認
    pub async fn find_validate_class_name(&self, input: InputClassEntity) -> Result<bool, Error> {
        let span = span!(
            Level::INFO,
            "db_query_execution",
            function = "find_validate_class_name"
        );
        let _enter = span.enter();

        let pool = self.0.get_pool();

        info!("Start validate class name");
        let classes = sqlx::query_scalar!(
            r#"
                SELECT EXISTS (
                SELECT 1
                FROM classes
                WHERE class_name = $1
                )
            "#,
            input.class_name,
        )
        .fetch_one(&pool)
        .await
        .map_err(|e| Error::DatabaseError(e))?;
        info!("Successful validate class name");

        match classes {
            Some(classes) => Ok(classes),
            None => return Err(Error::Unknown("query failure".to_string())),
        }
    }

    /// is_activeの更新
    pub async fn update_active(&self, input: InputUpdateClassEntity) -> Result<(), Error> {
        let span = span!(
            Level::INFO,
            "db_query_execution",
            function = "update_active"
        );
        let _enter = span.enter();

        let pool = self.0.get_pool();

        info!("Start updating is_active in the classes table");
        sqlx::query!(
            r#"
                UPDATE classes
                SET is_active = $1
                WHERE user_id = $2 AND class_name = $3
            "#,
            input.is_active,
            input.user_id.id(),
            input.class_name,
        )
        .execute(&pool)
        .await
        .map_err(|e| Error::DatabaseError(e))?;
        info!("Successfully updated is_active in classes table");

        Ok(())
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
            input.class_id.id(),
            input.user_id.id(),
        )
        .execute(&pool)
        .await
        .map_err(|e| Error::DatabaseError(e))?;
        info!("Successfully deleted classes table");

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{
        repository::users::tests::{users_create_test, users_delete_test, users_find_id},
        tests::util_init,
    };
    use ids_shared::Auth0Id;

    /// 作成テスト
    async fn classes_create_test() {
        let auth0_id = Auth0Id::from("test".to_string());
        let class_datas = vec![
            ("たまご", 0),
            ("ひよこ", 1),
            ("あひる", 2),
            ("うさぎ", 3),
            ("くま", 4),
            ("ぞう", 5),
        ];

        users_create_test(auth0_id.clone()).await;
        let user = users_find_id(auth0_id.clone()).await;
        let repo = ClassesRepository(util_init().await.unwrap());

        for class_data in class_datas {
            let input_class =
                InputClassEntity::new(user.id.clone(), class_data.0.to_string(), class_data.1);

            let class = repo.create(input_class).await;

            assert!(class.is_ok());
        }
    }

    /// アクティブなクラスの検索の正常系テスト
    async fn classes_find_active() {
        let auth0_id = Auth0Id::from("test".to_string());
        let user = users_find_id(auth0_id.clone()).await;
        let class_names = vec!["たまご", "ひよこ", "あひる", "うさぎ", "くま", "ぞう"];

        let repo = ClassesRepository(util_init().await.unwrap());

        let classes = repo.find_active_class(user.id).await.unwrap();

        for l in 0..classes.len() {
            assert_eq!(class_names[l].to_string(), classes[l].class_name);
        }
    }

    /// クラスネームの重複確認、正常系テスト
    async fn classes_validate_class_name() {
        let auth0_id = Auth0Id::from("test".to_string());
        let user = users_find_id(auth0_id.clone()).await;
        let class_data = ("ひよこ", 1);

        let repo = ClassesRepository(util_init().await.unwrap());
        let input = InputClassEntity::new(user.id, class_data.0.to_string(), class_data.1);

        let classe = repo.find_validate_class_name(input).await.unwrap();

        assert!(classe);
    }

    /// is_activeの更新、正常系テスト
    async fn classes_update_active() {
        let auth0_id = Auth0Id::from("test".to_string());
        let user = users_find_id(auth0_id.clone()).await;
        let class_data = ("ひよこ", 1);
        let is_active = false;

        let repo = ClassesRepository(util_init().await.unwrap());
        let input =
            InputUpdateClassEntity::new(user.id, class_data.0.to_string(), class_data.1, is_active);

        let classe = repo.update_active(input).await;

        assert!(classe.is_ok());
    }

    /// レコード削除、正常系テスト
    async fn classes_delete() {
        let auth0_id = Auth0Id::from("test".to_string());
        let user = users_find_id(auth0_id.clone()).await;

        let repo = ClassesRepository(util_init().await.unwrap());
        let classes = repo.find_all_class(user.id.clone()).await.unwrap();
        for class in classes {
            let input = InputDeleteClassEntity::new(user.id.clone(), class.id);
            let class_del = repo.delete(input).await;

            assert!(class_del.is_ok());
        }
        users_delete_test(auth0_id.clone()).await;
    }

    /// classes reporitoryの正常系テスト
    #[tokio::test]
    async fn classes_test() {
        classes_create_test().await;
        classes_find_active().await;
        classes_validate_class_name().await;
        classes_update_active().await;
        classes_delete().await;
    }
}
