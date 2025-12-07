// This file will have the code regarding JWT like creating and verifying it

use actix_web::cookie::time::Duration;
use chrono::{TimeDelta, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32, // the subject used to verify the member id in the db
    pub exp: i64, // the expiry of the token
}

pub fn generate_jwt(id: i32, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(TimeDelta::hours(1))
        .expect("Timestamp is invalid")
        .timestamp();

    let claims = Claims {
        sub: id.to_owned(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn decode_jwt(
    token: &str,
    secret: &str,
) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
}

pub fn is_valid(claim: &Claims) -> bool {
    claim.exp >= Utc::now().timestamp()
}
