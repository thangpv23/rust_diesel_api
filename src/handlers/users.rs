use warp::{Reply, Rejection};
use crate::db::DbPool;
use crate::models::user::User;
use crate::models::user::NewUser;
use diesel::prelude::*;
use serde::Deserialize;
use crate::auth::create_jwt;
use crate::middleware::InvalidToken;
use crate::schema::users::dsl::*;
use bcrypt::verify;
use serde_json;

#[derive(Deserialize)]
pub struct RegisterUser {
    username: String,
    password: String,
    email: String,
}

pub async fn register(pool: DbPool, body: RegisterUser) -> Result<impl Reply, Rejection> {
    use crate::schema::users::dsl::*;
    let mut conn = pool.get().map_err(|_| warp::reject::reject())?;

    let hashed = bcrypt::hash(&body.password, 12).map_err(|_| warp::reject::reject())?;

    let new_user = NewUser {
        username: &body.username,
        password_hash: &hashed,
        email: &body.email,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut conn)
        .map_err(|_| warp::reject::reject())?;

    Ok(warp::reply::json(&"User registered"))
}

#[derive(Deserialize)]
pub struct LoginUser {
    username: String,
    password: String,
}


pub async fn login(pool: DbPool, body: LoginUser) -> Result<impl warp::Reply, Rejection> {
    let mut conn = pool.get().map_err(|_| warp::reject::reject())?;

    let user = users
        .filter(username.eq(&body.username))
        .first::<User>(&mut conn)
        .optional()
        .map_err(|_| warp::reject::reject())?
        .ok_or_else(|| warp::reject::custom(InvalidCredentials))?;

    let valid = verify(&body.password, &user.password_hash)
        .map_err(|_| warp::reject::reject())?;

    if !valid {
        return Err(warp::reject::custom(InvalidCredentials));
    }

let token = create_jwt(user.id)
        .map_err(|_| warp::reject::custom(InvalidToken))?; // ← map JwtError → Rejection

    Ok(warp::reply::json(&serde_json::json!({ "token": token })))
}

#[derive(Debug)]
pub struct InvalidCredentials;
impl warp::reject::Reject for InvalidCredentials {}
