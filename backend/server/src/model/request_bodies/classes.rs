use serde::{Deserialize, Serialize};

/// POST api/v1/classes のリクエスト構造体
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequestPostClass {
    pub class_name: String,
    pub age: i32,
}

/// PUT api/v1/classes/:class_id のリクエスト構造体
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequestUpdateClass {
    pub class_name: String,
    pub age: i32,
}
