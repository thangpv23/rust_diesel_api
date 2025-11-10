use jsonwebtoken::{encode, decode, EncodingKey ,DecodingKey, Validation, errors::Error as JwtError};
use crate::models::jwt::Claims;
use std::env;

pub fn create_jwt(user_id: i32) -> Result<String, JwtError> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let claims = Claims::new(user_id, 24);
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn validate_jwt(token: &str) -> Result<Claims, JwtError> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

