use serde::Deserialize;

/// 環境変数
#[derive(Debug, Deserialize)]
pub struct KeyInitConfig {
    pub access_token_url: String,
    pub management_api_client_id: String,
    pub management_api_client_secret: String,
    pub management_api_audience: String,
    pub jwks_url: String,
}

/// 環境変数
#[derive(Debug, Deserialize)]
pub struct ValidateConfig {
    pub aud: String,
    pub iss: String,
}
