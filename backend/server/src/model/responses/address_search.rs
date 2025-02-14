use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResponseGetAddress {
    pub status: i32,
    pub message: Option<String>,
    pub results: Vec<Address>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub address1: String,
    pub address2: String,
    pub address3: String,
    pub kana1: String,
    pub kana2: String,
    pub kana3: String,
    pub prefcode: String,
    pub zipcode: String,
}
