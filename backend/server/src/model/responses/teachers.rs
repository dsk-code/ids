use ids_shared::types::id::TeacherId;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResponseTeacherId {
    pub id: TeacherId,
}
