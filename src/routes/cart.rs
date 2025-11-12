use warp::Filter;
use crate::db::connection::DbPool;
use crate::handlers::cart::*;
use crate::middleware::with_auth;
use crate::routes::common::pool::filter as pool_filter;

pub fn routes(pool: DbPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // POST /cart - Add item
    let add = warp::path!("cart")
        .and(warp::post())
        .and(with_auth())           // → user_id
        .and(pool_filter(pool.clone()))
        .and(warp::body::json())
        .and_then(add_to_cart);

    // GET /cart - View cart
    let view = warp::path!("cart")
        .and(warp::get())
        .and(with_auth())
        .and(pool_filter(pool.clone()))
        .and_then(view_cart);

    // DELETE /cart/items/:item_id
    let remove = warp::path!("cart" / "items" / i32)
        .and(warp::delete())
        .and(with_auth())
        .and(pool_filter(pool))
        .and_then(remove_from_cart);

    add.or(view).or(remove)
}