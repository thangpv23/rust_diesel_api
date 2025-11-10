use crate::auth::validate_jwt;
use warp::{Filter, Rejection};

pub fn with_auth() -> impl Filter<Extract = (i32,), Error = Rejection> + Clone {
    warp::header::optional::<String>("authorization")
        .and_then(|auth: Option<String>| async move {
            let token = auth
                .and_then(|h| h.strip_prefix("Bearer ").map(|s| s.to_string()))
                .ok_or_else(|| warp::reject::custom(InvalidToken))?;

            let claims = validate_jwt(&token)
                .map_err(|_| warp::reject::custom(InvalidToken))?; // ← map JwtError → Rejection

            Ok::<_, Rejection>(claims.sub)
        })
}

#[derive(Debug)]
pub struct InvalidToken;
impl warp::reject::Reject for InvalidToken {}