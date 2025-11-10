// Cart handlers
use warp::{Reply, Rejection};
use crate::db::DbPool;
// Add models, diesel, serde as needed
use crate::models::cart::Cart;
use crate::models::cart::NewCart;
use crate:: models::cart::CartItem;
use crate:: models::cart::NewCartItem;
use diesel::prelude::*;


pub async fn create_cart(pool: DbPool, uid: i32) -> Result<impl Reply, Rejection> {
    // Implement logic
    use crate::schema::carts::dsl::*;
    let mut conn = pool.get().map_err(|_| warp::reject::reject())?;

    let new_cart = NewCart {
        user_id: uid,
    };

    diesel::insert_into(carts)
        .values(&new_cart)
        .execute(&mut conn)
        .map_err(|_| warp::reject::reject())?;

    Ok(warp::reply::json(&"Cart created"))
}

pub async fn add_item_to_cart(pool: DbPool, add_cart_id: i32, add_item_id: i32, add_quantity: i32) -> Result<impl Reply, Rejection> {
    // Implement logic to add item to cart
     use crate::schema::cart_items::dsl::*;
    let mut conn = pool.get().map_err(|_| warp::reject::reject())?;
    
    let new_cart_item = NewCartItem {
        cart_id : add_cart_id,
        item_id : add_item_id,
        quantity : add_quantity,
    };

    diesel::insert_into(cart_items)
        .values(&new_cart_item)
        .execute(&mut conn)
        .map_err(|_| warp::reject::reject())?;

    Ok(warp::reply::json(&"Item added to cart"))
}

pub async fn remove_cart(pool: DbPool, cart_id: i32) -> Result<impl Reply, Rejection> {
    use crate::schema::carts::dsl::*;
    let mut conn = pool.get().map_err(|_| warp::reject::reject())?;

    diesel::delete(carts.filter(id.eq(cart_id)))
        .execute(&mut conn)
        .map_err(|_| warp::reject::reject())?;

    Ok(warp::reply::json(&"Cart removed"))
}

pub async fn remove_cart_item(pool: DbPool, remove_cart_id: i32, remove_item_id: i32) -> Result<impl Reply, Rejection> {
    use crate::schema::cart_items::dsl::*;
    let mut conn = pool.get().map_err(|_| warp::reject::reject())?;

    diesel::delete(
        cart_items.filter(
            cart_id.eq(remove_cart_id).and(item_id.eq(remove_item_id))
        )
    )
    .execute(&mut conn)
    .map_err(|_| warp::reject::reject())?;

    Ok(warp::reply::json(&"Item removed from cart"))
}

pub async fn edit_cart_item(pool: DbPool, edit_cart_id: i32, edit_item_id: i32, new_quantity: i32) -> Result<impl Reply, Rejection> {
    use crate::schema::cart_items::dsl::*;
    let mut conn = pool.get().map_err(|_| warp::reject::reject())?;

    diesel::update(
        cart_items.filter(
            cart_id.eq(edit_cart_id).and(item_id.eq(edit_item_id))
        )
    )
    .set(quantity.eq(new_quantity))
    .execute(&mut conn)
    .map_err(|_| warp::reject::reject())?;

    Ok(warp::reply::json(&"Cart item updated"))
}
