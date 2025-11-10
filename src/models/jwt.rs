use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,           // user id
    pub exp: usize,         // expiration timestamp
}

impl Claims {
    pub fn new(user_id: i32, ttl_hours: i64) -> Self {
        let exp = (Utc::now() + Duration::hours(ttl_hours)).timestamp() as usize;
        Self { sub: user_id, exp }
    }
}