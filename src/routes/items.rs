// Items CRUD routes
use warp::Filter;
use crate::db::connection::DbPool;
use crate::middleware::with_auth;
use crate::handlers::items;
use crate::routes::common::pool::filter as pool_filter;


pub fn routes(pool: DbPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("items")
        .and(warp::get())
        // Add handlers
        .map(|| "Items endpoint")
}

pub fn get_list_items (pool: DbPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("items")
        .and(warp::get())
        .and(with_auth())          
        .and(pool_filter(pool))    
        .and_then(items::list_items)     
}