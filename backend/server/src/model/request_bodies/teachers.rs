use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequestPostTeacher {
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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequestPutTeacher {
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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequestPatchTeacher {
    pub leave_date: Option<NaiveDate>,
    pub status: String,
}
