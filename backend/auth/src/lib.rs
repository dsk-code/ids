pub mod error;
use error::AuthError as Error;

use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use uuid::Uuid;

static KEYS: OnceLock<KeySet> = OnceLock::new();
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct JWT(String);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Claims {
    iss: String, // issuer (JWTの発行者)
    sub: Uuid,   // subject (ユーザーの識別子)
    aud: String, // audience (JWTの受信者)
    exp: i64,    // expiration time (有効期限)
    iat: i64,    // Issued At (発行日時)
    jti: Uuid,   // JWT ID (JWTの一意な識別子)
}

struct KeySet {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JWT {
    pub fn new(token: String) -> Self {
        Self(token)
    }

    /// returns a token reference
    pub fn access_token(&self) -> &str {
        &self.0
    }

    /// Create a token and return JWT with Result type
    pub fn create(iss: String, sub: Uuid, aud: String, duration_hours: i64) -> Result<Self, Error> {
        let iat: DateTime<Utc> = Utc::now();
        let exp: DateTime<Utc> = iat + Duration::hours(duration_hours);
        let claims = Claims {
            iss,
            sub,
            aud,
            exp: exp.timestamp(),
            iat: iat.timestamp(),
            jti: Uuid::new_v4(),
        };
        let header: Header = Header::new(Algorithm::HS512);
        let encoding_key = &KEYS.get().ok_or(Error::KeySetFailure)?.encoding_key;
        let token: String = encode(&header, &claims, encoding_key)
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(JWT(token))
    }

    /// Validate the token and return Claims with Result type
    pub fn validate(&self, aud: &str) -> Result<Claims, Error> {
        let decoding_key = &KEYS.get().ok_or(Error::KeySetFailure)?.decoding_key;
        let mut validation: Validation = Validation::new(Algorithm::HS512);
        validation.set_audience(&[aud]);
        let token_data = decode::<Claims>(self.access_token(), &decoding_key, &validation)
            .map_err(|e| Error::EncodeError(e.to_string()))?;

        Ok(token_data.claims)
    }
}

/// Initialize the key
pub fn key_init(secret: &[u8]) -> Result<(), Error> {
    let key_set = KeySet {
        encoding_key: EncodingKey::from_secret(secret),
        decoding_key: DecodingKey::from_secret(secret),
    };

    KEYS.set(key_set).map_err(|_| Error::KeySetFailure)
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     // #[test]
//     // fn key_init_test() {
//     //     let secret = "abcdefg".as_ref();
//     //     let key_set = key_init(secret);

//     //     assert!(key_set.is_ok());
//     // }
//     // #[test]
//     // fn failed_key_init_test2() {
//     //     let secret = "hijklmn".as_ref();
//     //     let key_set = key_init(secret);

//     //     assert!(key_set.is_err());
//     // }

//     #[test]
//     fn jwt_create_test() {
//         let iss = "example@example.com".to_string();
//         let sub = Uuid::new_v4();
//         let aud = "example@example.com/test".to_string();
//         let duration_hours = 2;

//         let secret = "abcdefg".as_ref();
//         key_init(secret).unwrap();

//         let jwt = JWT::create(iss, sub, aud, duration_hours);
//         match jwt {
//             Ok(_) => println!("Successfully created JWT"),
//             Err(ref e) => eprintln!("AuthError: {}", e),
//         }
//         assert!(jwt.is_ok());
//     }

//     // #[test]
//     // fn validate_test() {
//     //     let iss = "example@example.com".to_string();
//     //     let sub = Uuid::new_v4();
//     //     let aud = "example@example.com/test".to_string();
//     //     let duration_hours = 2;

//     //     let token = JWT::create(iss.clone(), sub.clone(), aud.clone(), duration_hours)
//     //         .unwrap()
//     //         .validate(&aud)
//     //         .unwrap();

//     //     assert_eq!(iss, token.iss);
//     //     assert_eq!(sub, token.sub);
//     // }

//     // #[test]
//     // fn should_fail_validate_test() {
//     //     let iss = "example@example.com".to_string();
//     //     let sub = Uuid::new_v4();
//     //     let aud = "example@example.com/test".to_string();
//     //     let duration_hours = 2;
//     //     let fail_aud = "example@example.com/fail".to_string();

//     //     let token = JWT::create(iss.clone(), sub.clone(), aud.clone(), duration_hours)
//     //         .unwrap()
//     //         .validate(&fail_aud)
//     //         ;

//     //     assert!(token.is_err());

//     // }

// }
