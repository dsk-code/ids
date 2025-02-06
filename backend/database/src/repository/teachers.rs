use super::Pagination;
use crate::{error::Error, DbConnector};

use ids_shared::{types::id::TeacherId, UserId};
use tracing::{info, span, Level};

use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};
use derive_new::new;
use std::sync::Arc;

#[derive(Debug, new)]
pub struct TeacherEntity {
    pub id: TeacherId,
    pub user_id: UserId,
    pub last_name: String,
    pub first_name: String,
    pub last_name_kana: Option<String>,
    pub first_name_kana: Option<String>,
    pub phone: Option<String>,
    pub mobile_phone: Option<String>,
    pub email: Option<String>,
    pub post_code1: String,
    pub post_code2: String,
    pub prefecture: String,
    pub city: String,
    pub street_address: String,
    pub building: Option<String>,
    pub prefectures_kana: Option<String>,
    pub city_kana: Option<String>,
    pub building_kana: Option<String>,
    pub hire_date: NaiveDate,
    pub leave_date: Option<NaiveDate>,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, new)]
pub struct PaginatedTeachersListEntity {
    pub teachers: Vec<TeacherEntity>,
    pub offset: i64,
    pub limit: i64,
    pub total: Option<i64>,
}

#[derive(Debug, new)]
pub struct InputTeacherEntity {
    pub id: TeacherId,
    pub user_id: UserId,
    pub last_name: String,
    pub first_name: String,
    pub last_name_kana: Option<String>,
    pub first_name_kana: Option<String>,
    pub phone: Option<String>,
    pub mobile_phone: Option<String>,
    pub email: Option<String>,
    pub post_code1: String,
    pub post_code2: String,
    pub prefecture: String,
    pub city: String,
    pub street_address: String,
    pub building: Option<String>,
    pub prefectures_kana: Option<String>,
    pub city_kana: Option<String>,
    pub building_kana: Option<String>,
    pub hire_date: NaiveDate,
}

#[derive(Debug, new)]
pub struct InputUpdateTeacherEntity {
    pub id: TeacherId,
    pub user_id: UserId,
    pub last_name: String,
    pub first_name: String,
    pub last_name_kana: Option<String>,
    pub first_name_kana: Option<String>,
    pub phone: Option<String>,
    pub mobile_phone: Option<String>,
    pub email: Option<String>,
    pub post_code1: String,
    pub post_code2: String,
    pub prefecture: String,
    pub city: String,
    pub street_address: String,
    pub building: Option<String>,
    pub prefectures_kana: Option<String>,
    pub city_kana: Option<String>,
    pub building_kana: Option<String>,
    pub hire_date: NaiveDate,
}

#[derive(Debug, new)]
pub struct InputFindTeacherEntity {
    pub id: TeacherId,
    pub user_id: UserId,
}

#[derive(Debug, new)]
pub struct InputPatchTeacherEntity {
    pub id: TeacherId,
    pub user_id: UserId,
    pub leave_date: Option<NaiveDate>,
    pub status: String,
}

#[derive(Debug, new)]
pub struct InputDeleteTeacherEntity {
    pub id: TeacherId,
    pub user_id: UserId,
}

#[async_trait]
pub trait TeachersRepository {
    async fn create(&self, input: InputTeacherEntity) -> Result<TeacherId, Error>;
    async fn find_all_with_pagination(
        &self,
        input: UserId,
        pagination: Pagination,
    ) -> Result<PaginatedTeachersListEntity, Error>;
    async fn find_teacher(&self, input: InputFindTeacherEntity) -> Result<TeacherEntity, Error>;
    async fn update(&self, input: InputUpdateTeacherEntity) -> Result<TeacherId, Error>;
    async fn status_update(&self, input: InputPatchTeacherEntity) -> Result<TeacherId, Error>;
    async fn delete(&self, input: InputDeleteTeacherEntity) -> Result<(), Error>;
}

#[derive(Clone)]
pub struct PostgresTeachersRepository(Arc<DbConnector>);

impl PostgresTeachersRepository {
    pub fn new(db: Arc<DbConnector>) -> Self {
        Self(db)
    }
}

#[async_trait]
impl TeachersRepository for PostgresTeachersRepository {
    async fn create(&self, input: InputTeacherEntity) -> Result<TeacherId, Error> {
        let span = span!(Level::INFO, "db_query_execution", function = "create");
        let _enter = span.enter();

        let pool = self.0.get_pool();

        info!("New teacher registration started");
        let id = sqlx::query_scalar!(
            r#"
                INSERT INTO teachers
                    (id, user_id, last_name, first_name, last_name_kana, first_name_kana, phone, mobile_phone, email, post_code1, post_code2, prefecture, city, street_address, building, prefectures_kana, city_kana, building_kana, hire_date, status)
                VALUES
                    ($1::UUID, $2::UUID, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, 'active')
                RETURNING id    
            "#,
            input.id.id(),
            input.user_id.id(),
            input.last_name,
            input.first_name,
            input.last_name_kana,
            input.first_name_kana,
            input.phone,
            input.mobile_phone,
            input.email,
            input.post_code1,
            input.post_code2,
            input.prefecture,
            input.city,
            input.street_address,
            input.building,
            input.prefectures_kana,
            input.city_kana,
            input.building_kana,
            input.hire_date,
        )
        .fetch_one(&pool)
        .await
        .map_err(Error::DatabaseError)?;
        info!("Successful new teacher registration");

        Ok(TeacherId::from(id))
    }

    async fn find_all_with_pagination(
        &self,
        input: UserId,
        pagination: Pagination,
    ) -> Result<PaginatedTeachersListEntity, Error> {
        let span = span!(
            Level::INFO,
            "db_query_execution",
            function = "find_all_with_pagination"
        );
        let _enter = span.enter();

        let pool = self.0.get_pool();

        info!("Start find_all_with_pagination TeachersEntity");
        let teachers = sqlx::query_as!(
            TeacherEntity,
            r#"
                SELECT 
                    id,
                    user_id,
                    last_name,
                    first_name,
                    last_name_kana,
                    first_name_kana,
                    phone,
                    mobile_phone,
                    email,
                    post_code1,
                    post_code2,
                    prefecture,
                    city,
                    street_address,
                    building,
                    prefectures_kana,
                    city_kana,
                    building_kana,
                    hire_date,
                    leave_date,
                    status,
                    created_at,
                    updated_at
                FROM teachers
                WHERE user_id = $1::UUID
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
            "#,
            input.id(),
            pagination.limit,
            pagination.offset,
        )
        .fetch_all(&pool)
        .await
        .map_err(Error::DatabaseError)?;

        let total = sqlx::query_scalar!(
            r#"
                SELECT COUNT(*)
                FROM teachers
            "#
        )
        .fetch_one(&pool)
        .await
        .map_err(Error::DatabaseError)?;
        info!("Successful search for TeachersEntity");

        Ok(PaginatedTeachersListEntity::new(
            teachers,
            pagination.offset,
            pagination.limit,
            total,
        ))
    }

    async fn find_teacher(&self, input: InputFindTeacherEntity) -> Result<TeacherEntity, Error> {
        let span = span!(Level::INFO, "db_query_execution", function = "find_teacher");
        let _enter = span.enter();

        let pool = self.0.get_pool();

        info!("Start find TeacherEntity");
        let teacher = sqlx::query_as!(
            TeacherEntity,
            r#"
                SELECT 
                    id,
                    user_id,
                    last_name,
                    first_name,
                    last_name_kana,
                    first_name_kana,
                    phone,
                    mobile_phone,
                    email,
                    post_code1,
                    post_code2,
                    prefecture,
                    city,
                    street_address,
                    building,
                    prefectures_kana,
                    city_kana,
                    building_kana,
                    hire_date,
                    leave_date,
                    status,
                    created_at,
                    updated_at
                FROM teachers
                WHERE id = $1::UUID AND user_id = $2::UUID
            "#,
            input.id.id(),
            input.user_id.id()
        )
        .fetch_one(&pool)
        .await
        .map_err(Error::DatabaseError)?;
        info!("Successful search for TeacherEntity");

        Ok(teacher)
    }

    async fn update(&self, input: InputUpdateTeacherEntity) -> Result<TeacherId, Error> {
        let span = span!(Level::INFO, "db_query_execution", function = "update");
        let _enter = span.enter();

        let pool = self.0.get_pool();

        info!("Start update TeacherEntity");
        let id = sqlx::query_scalar!(
            r#"
                UPDATE teachers
                SET 
                    last_name = $1,
                    first_name = $2,
                    last_name_kana = $3,
                    first_name_kana = $4,
                    phone = $5,
                    mobile_phone = $6,
                    email = $7,
                    post_code1 = $8,
                    post_code2 = $9,
                    prefecture = $10,
                    city = $11,
                    street_address = $12,
                    building = $13,
                    prefectures_kana = $14,
                    city_kana = $15,
                    building_kana = $16,
                    hire_date = $17
                WHERE user_id = $18::UUID AND id = $19::UUID
                RETURNING id
            "#,
            input.last_name,
            input.first_name,
            input.last_name_kana,
            input.first_name_kana,
            input.phone,
            input.mobile_phone,
            input.email,
            input.post_code1,
            input.post_code2,
            input.prefecture,
            input.city,
            input.street_address,
            input.building,
            input.prefectures_kana,
            input.city_kana,
            input.building_kana,
            input.hire_date,
            input.user_id.id(),
            input.id.id(),
        )
        .fetch_one(&pool)
        .await
        .map_err(Error::DatabaseError)?;
        info!("Successful update TeacherEntity");

        Ok(TeacherId::from(id))
    }

    async fn status_update(&self, input: InputPatchTeacherEntity) -> Result<TeacherId, Error> {
        let span = span!(
            Level::INFO,
            "db_query_execution",
            function = "status_update"
        );
        let _enter = span.enter();

        let pool = self.0.get_pool();

        info!("Start status_update TeacherEntity");
        let id = sqlx::query_scalar!(
            r#"
                UPDATE teachers
                SET leave_date = $1, status = $2
                WHERE user_id = $3::UUID AND id = $4::UUID
                RETURNING id
            "#,
            input.leave_date,
            input.status,
            input.user_id.id(),
            input.id.id(),
        )
        .fetch_one(&pool)
        .await
        .map_err(Error::DatabaseError)?;
        info!("Successful status_update TeacherEntity");

        Ok(TeacherId::from(id))
    }

    async fn delete(&self, input: InputDeleteTeacherEntity) -> Result<(), Error> {
        let span = span!(Level::INFO, "db_query_execution", function = "delete");
        let _enter = span.enter();

        let pool = self.0.get_pool();

        info!("Start delete TeacherEntity");
        sqlx::query!(
            r#"
                DELETE FROM teachers WHERE id = $1::UUID AND user_id = $2::UUID
            "#,
            input.id.id(),
            input.user_id.id(),
        )
        .execute(&pool)
        .await
        .map_err(Error::DatabaseError)?;
        info!("Successful delete TeacherEntity");

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
        let repo = PostgresTeachersRepository::new(Arc::new(db));

        let user = test_util_create_user(pool).await;

        let input = InputTeacherEntity {
            id: TeacherId::new_v4(),
            user_id: user.id,
            last_name: (1..=50).fake::<String>(),
            first_name: (1..=50).fake::<String>(),
            last_name_kana: Some("カタカナ".to_string()),
            first_name_kana: Some("カタカナ".to_string()),
            phone: Some((1..=15).fake::<String>()),
            mobile_phone: Some((1..=15).fake::<String>()),
            email: Some((1..=255).fake::<String>()),
            post_code1: "666".to_string(),
            post_code2: "6666".to_string(),
            prefecture: (1..=30).fake::<String>(),
            city: (1..=40).fake::<String>(),
            street_address: (1..=100).fake::<String>(),
            building: Some((1..=50).fake::<String>()),
            prefectures_kana: Some("カタカナ".to_string()),
            city_kana: Some("カタカナ".to_string()),
            building_kana: Some("カタカナ".to_string()),
            hire_date: Faker.fake::<NaiveDate>(),
        };

        // test
        let res = repo.create(input).await;

        assert!(res.is_ok());

        Ok(())
    }
}
