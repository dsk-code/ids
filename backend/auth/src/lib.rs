pub mod client;
pub mod error;
pub mod types;

use crate::client::{ManageMentAccessToken, Jwks};
use crate::error::AuthError;
use crate::types::{KeyInitConfig, ValidateConfig};

use std::sync::OnceLock;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

static KEYS: OnceLock<Jwks> = OnceLock::new();

/// アクストークンのClaimsを表現する構造体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub iat: u64,
    pub exp: u64,
    pub gty: String,
    pub azp: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct JWT(String);

impl JWT {
    pub fn new(token: String) -> Self {
        Self(token)
    }

    pub fn access_token(&self) -> &str {
        &self.0
    }

    /// アクセストークンを検証
    pub fn validate(&self, secret: &ValidateConfig) -> Result<Claims, AuthError> {
        let jwks = KEYS.get().ok_or(AuthError::NotFound("key".to_string()))?;
        let header = decode_header(self.access_token())?;
        let jwk = jwks.get_jwk(&header.kid.ok_or(AuthError::NotFound("kid".to_string()))?)?;
        let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)?;
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[&secret.aud]);
        validation.set_issuer(&[&secret.iss]);
        let token_data = decode::<Claims>(self.access_token(), &decoding_key, &validation)?;

        Ok(token_data.claims)
    }
}

/// Initialize the key
pub async fn key_init(secret: &KeyInitConfig) -> Result<(), AuthError> {
    // 管理APIアクセストークン取得
    let access_token = ManageMentAccessToken::get_access_token(
        secret.access_token_url.clone(), 
        secret.management_api_client_id.clone(), 
        secret.management_api_client_secret.clone(), 
        secret.management_api_audience.clone()
    ).await?;
    // JWKS取得
    let jwks = Jwks::new(
        secret.jwks_url.clone(), 
        access_token
    ).await?;

    KEYS.set(jwks).map_err(|_| AuthError::InvalidKeyset)
}

/// 正常系テスト
#[cfg(test)]
mod tests {
    use std::{collections::HashMap, time::{Duration, UNIX_EPOCH}};
    use reqwest::{header::CONTENT_TYPE, Client};
    use super::*;

    /// テストのためのトークンを取得するための環境変数
    #[derive(Debug, Deserialize)]
    pub struct TestTokenEnvConfig {
    pub access_token_url: String,
    pub app_api_client_id: String,
    pub app_api_client_secret: String,
    pub app_api_audience: String,
    }

    /// テストのためのAuth0APIのアクセストークンのレスポンスを表現する構造体
    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    pub struct AppAccessToken {
        pub access_token: String,
        pub token_type: String,
        pub expires_in: i32,
    }

    impl AppAccessToken {
        /// テストのためのアクセストークン取得
        pub async fn get_access_token(url: String, client_id: String, client_secret: String, audience: String) -> Result<Self, reqwest::Error> {
            let client = Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to build client");

            let mut params = HashMap::new();
            params.insert("grant_type".to_string(), "client_credentials".to_string());
            params.insert("client_id".to_string(), client_id);
            params.insert("client_secret".to_string(), client_secret);
            params.insert("audience".to_string(), audience);

            let response = client
                .post(&url)
                .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                .form(&params)
                .send()
                .await?
                .json::<AppAccessToken>()
                .await?;

            Ok(response)
        }
    }

    /// 初期化の為の環境変数を返す
    fn key_init_env() -> KeyInitConfig {
        dotenvy::dotenv().expect("Failed to get environment variables");
        let secret = envy::from_env::<KeyInitConfig>()
            .expect("Failed to deserialize environment variables");

        secret
    }

    /// 検証の為の環境変数を返す
    fn validate_env() -> ValidateConfig {
        dotenvy::dotenv().expect("Failed to get environment variables");
        let secret = envy::from_env::<ValidateConfig>()
            .expect("Failed to deserialize environment variables");

        secret
    }

    /// テストの為のトークンを取得するための環境変数を返す
    fn token_env() -> TestTokenEnvConfig {
        dotenvy::dotenv().expect("Failed to get environment variables");
        let secret = envy::from_env::<TestTokenEnvConfig>()
            .expect("Failed to deserialize environment variables");

        secret
    }

    /// 有効期限が切れていない場合はfalse, 切れている場合はtrueを返す
    fn is_expected_time(exp: u64) -> bool {
        let current_timestamp = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards").as_secs();
        exp <= current_timestamp
    }

    /// 鍵の初期化の正常テスト
    #[tokio::test]
    async fn test_key_init() {
        let secret = key_init_env();
        let result = key_init(&secret).await;

        assert!(result.is_ok());
    }

    /// Jwt検証の正常テスト
    #[tokio::test]
    async fn test_jwt() {
        let key_init_secret = key_init_env();
        let token_secret = token_env();
        let validate_secret = validate_env();
        key_init(&key_init_secret).await.unwrap();
        let token = AppAccessToken::get_access_token(
                token_secret.access_token_url.clone(), 
                token_secret.app_api_client_id.clone(), 
                token_secret.app_api_client_secret.clone(), 
                token_secret.app_api_audience.clone()
            )
            .await
            .expect("Failed to obtain access token");
        let jwt = JWT::new(token.access_token);
        let result = jwt.validate(&validate_secret).unwrap();

        assert_eq!(result.aud, validate_secret.aud);
        assert_eq!(result.iss, validate_secret.iss);
        assert!(!is_expected_time(result.exp));
    }
}
