// Returns impl Filter<Extract = impl Reply, Error = Rejection> + Clone

use warp::Filter;
use crate::db::connection::DbPool;
use crate::handlers::users::*;
use crate::routes::common::pool::filter as pool_filter;

pub fn routes(pool: DbPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // create new user
    let register = warp::path!("users" / "register")
        .and(warp::post())
        .and(pool_filter(pool.clone()))
        .and(warp::body::json())
        .and_then(register);

    // authenticate and return JWT
    let login = warp::path!("users" / "login")
        .and(warp::post())
        .and(pool_filter(pool))
        .and(warp::body::json())
        .and_then(login); 


    // Combine all user routes
    register.or(login)
}