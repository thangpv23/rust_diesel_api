use warp::Filter;
use crate::db::connection::DbPool;
use crate::handlers::users;

pub fn routes(pool: DbPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("users" / "register")
        .and(warp::post())
        .and(pool_filter(pool.clone()))
        .and(warp::body::json())
        .and_then(users::register)
}

// Helper to clone pool
fn pool_filter(pool: DbPool) -> impl Filter<Extract = (DbPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}