use ids_shared::{ClassId, UserId};

use serde::{Deserialize, Serialize};

/// POST api/v1/classes のリクエスト構造体
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequestPostClass {
    pub class_name: String,
    pub age: i32,
}

/// GET api/v1/classes/:class_id のリクエスト構造体
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequestGetClass {
    pub id: ClassId,
    pub user_id: UserId,
}

/// PUT api/v1/classes/:class_id のリクエスト構造体
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequestUpdateClass {
    pub id: ClassId,
    pub user_id: UserId,
    pub class_name: String,
    pub age: i32,
}

/// DELETE api/v1/classes/:class_id のリクエスト構造体
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequestDeleteClass {
    pub id: ClassId,
    pub user_id: UserId,
}
