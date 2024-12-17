use serde::{Deserialize, Serialize};

/// POST api/vi/me
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestAuthUser {
    pub user_name: String,
    pub user_email: String,
}
