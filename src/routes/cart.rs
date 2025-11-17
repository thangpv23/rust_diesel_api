use warp::Filter;
use crate::db::connection::DbPool;
use crate::handlers::cart::*;
use crate::middleware::with_auth;
use crate::routes::common::pool::filter as pool_filter;

pub fn routes(pool: DbPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Routes for /cart/items/:item_id (more specific, check first)
    let remove = warp::path!("cart" / "items" / i32)
        .and(warp::path::end())
        .and(warp::delete())
        .and(with_auth())
        .and(pool_filter(pool.clone()))
        .and_then(remove_from_cart);

    let update = warp::path!("cart" / "items" / i32)
        .and(warp::path::end())
        .and(warp::put())
        .and(with_auth())
        .and(pool_filter(pool.clone()))
        .and(warp::body::json())
        .and_then(update_item_quantity);

    // Routes for /cart (less specific, check after)
    let add = warp::path!("cart")
        .and(warp::path::end())
        .and(warp::post())
        .and(with_auth())
        .and(pool_filter(pool.clone()))
        .and(warp::body::json())
        .and_then(add_to_cart);

    let view = warp::path!("cart")
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth())
        .and(pool_filter(pool.clone()))
        .and_then(view_cart);

    let clear = warp::path!("cart")
        .and(warp::path::end())
        .and(warp::delete())
        .and(with_auth())
        .and(pool_filter(pool.clone()))
        .and_then(clear_cart);

    // Combine: more specific routes first
    remove.or(update).or(add).or(view).or(clear)
}