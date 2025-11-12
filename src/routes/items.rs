// All routes return impl Filter<Extract = impl Reply, Error = Rejection> + Clone

use warp::Filter;
use crate::db::connection::DbPool;
use crate::handlers::items::*;
use crate::middleware::with_auth;
use crate::routes::common::pool::filter as pool_filter;

pub fn routes(pool: DbPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // 1. GET /items – PUBLIC: list all items (no auth required)
    let list_public = warp::path!("items")
        .and(warp::get())
        .and(pool_filter(pool.clone()))
        .and_then(list_items); 

    // 2. POST /items – PROTECTED: create new item
    //    Requires: JWT → user_id, JSON body → NewItem
    let create = warp::path!("items")
        .and(warp::post())
        .and(with_auth())                    
        .and(pool_filter(pool.clone()))      
        .and(warp::body::json())            
        .and_then(create_item);              

    // 3. GET /items/:id – PUBLIC: read single item by ID

    let read_one = warp::path!("items" / i32)        
        .and(warp::get())
        .and(with_auth())     
        .and(pool_filter(pool.clone()))
        .and_then(read_item);                    

    // 4. PUT /items/:id – PROTECTED: update item

    let update = warp::path!("items" / i32)
        .and(warp::put())
        .and(with_auth())
        .and(pool_filter(pool.clone()))
        .and(warp::body::json())
        .and_then(update_item);                  

    // 5. DELETE /items/:id – PROTECTED: delete item

    let delete = warp::path!("items" / i32)
        .and(warp::delete())
        .and(with_auth())
        .and(pool_filter(pool))
        .and_then(delete_item);                 

    // Combine all routes

    list_public
        .or(create)
        .or(read_one)
        .or(update)
        .or(delete)
}