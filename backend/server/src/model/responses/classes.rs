use ids_shared::{UserId, ClassId};

use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

/// api/v1/classes のレスポンス構造体
/// api/v1/classes/:class_id のレスポンス構造体
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResponseClass {
    pub id: ClassId,
    pub user_id: UserId,
    pub class_name: String,
    pub age: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
