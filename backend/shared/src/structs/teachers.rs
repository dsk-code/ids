use chrono::{NaiveDate, NaiveDateTime};

use crate::{types::id::TeacherId, UserId};

pub struct Teacher {
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
    pub prefectures: String,
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
