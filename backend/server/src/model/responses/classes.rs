use ids_shared::{UserId, ClassId};

use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;


#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClassEntity {
    pub id: ClassId,
    pub user_id: UserId,
    pub class_name: String,
    pub age: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
