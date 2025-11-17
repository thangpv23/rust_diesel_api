// Cart handlers – 1 cart per user, auto-managed via JWT user_id

use warp::{Reply, Rejection};
use crate::db::connection::DbPool;
use crate::models::cart::{Cart, NewCart, CartItem, NewCartItem};
use crate::schema::carts::dsl::{carts, id as cart_id, user_id as cart_user_id};    
use crate::schema::cart_items::dsl::{cart_items, cart_id as ci_cart_id, item_id as ci_item_id}; 
use diesel::prelude::*;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct AddToCart {
    pub item_id: i32,
    pub quantity: i32,
}

// Accept either { "quantity": 5 } or just 5
#[derive(Deserialize)]
#[serde(untagged)]
pub enum QuantityPayload {
    Object { quantity: i32 },
    Number(i32),
}

impl QuantityPayload {
    pub fn value(self) -> i32 {
        match self {
            QuantityPayload::Object { quantity } => quantity,
            QuantityPayload::Number(q) => q,
        }
    }
}

// Helper: Get or create cart for user
async fn get_or_create_cart(conn: &mut PgConnection, uid: i32) -> Result<i32, diesel::result::Error> {
    let existing_cart_id: Option<i32> = carts
        .filter(cart_user_id.eq(uid))
        .select(cart_id)  
        .first(conn)
        .optional()?;

    if let Some(id) = existing_cart_id {
        return Ok(id);
    }

    // Create new cart
    let new_cart = NewCart { user_id: uid };
    let new_id: i32 = diesel::insert_into(carts)
        .values(&new_cart)
        .returning(cart_id) 
        .get_result(conn)?;

    Ok(new_id)
}

pub async fn add_to_cart(
    user_id: i32,
    pool: DbPool,
    body: AddToCart,
) -> Result<impl Reply, Rejection> {
    let mut conn = pool.get().map_err(|_| warp::reject::reject())?;
    let cart_result_id = get_or_create_cart(&mut conn, user_id)
        .await
        .map_err(|_| warp::reject::reject())?;

    let new_item = NewCartItem {
        cart_id: cart_result_id,
        item_id: body.item_id,
        quantity: body.quantity,
    };

    diesel::insert_into(cart_items)
        .values(&new_item)
        .on_conflict((ci_cart_id, ci_item_id))
        .do_update()
        .set(crate::schema::cart_items::dsl::quantity.eq(crate::schema::cart_items::dsl::quantity + body.quantity))
        .execute(&mut conn)
        .map_err(|_| warp::reject::reject())?;

    Ok(warp::reply::json(&json!({ "message": "Item added to cart" })))
}

pub async fn view_cart(user_id: i32, pool: DbPool) -> Result<impl Reply, Rejection> {
    let mut conn = pool.get().map_err(|_| warp::reject::reject())?;
    let cart_result_id = get_or_create_cart(&mut conn, user_id)
        .await
        .map_err(|_| warp::reject::reject())?;

    let items = cart_items
        .filter(ci_cart_id.eq(cart_result_id))  
        .load::<CartItem>(&mut conn)
        .map_err(|_| warp::reject::reject())?;

    Ok(warp::reply::json(&items))
}

pub async fn remove_from_cart(
    item_id: i32,
    user_id: i32,
    pool: DbPool,
) -> Result<impl Reply, Rejection> {
    let mut conn = pool.get().map_err(|_| warp::reject::reject())?;
    let cart_result_id = get_or_create_cart(&mut conn, user_id)
        .await
        .map_err(|_| warp::reject::reject())?;

    diesel::delete(
        cart_items.filter(ci_cart_id.eq(cart_result_id).and(ci_item_id.eq(item_id)))
    )
    .execute(&mut conn)
    .map_err(|_| warp::reject::reject())?;

    Ok(warp::reply::json(&json!({ "message": "Item removed from cart" })))
}


pub async fn clear_cart(
    user_id: i32,
    pool: DbPool,
) -> Result<impl Reply, Rejection> {
    let mut conn = pool.get().map_err(|_| warp::reject::reject())?;
    let cart_result_id = get_or_create_cart(&mut conn, user_id)
        .await
        .map_err(|_| warp::reject::reject())?;

    diesel::delete(
        cart_items.filter(ci_cart_id.eq(cart_result_id))
    )
    .execute(&mut conn)
    .map_err(|_| warp::reject::reject())?;

    Ok(warp::reply::json(&json!({ "message": "Cart cleared" })))
}

pub async fn update_item_quantity(
    item_id: i32,
    user_id: i32,
    pool: DbPool,
    payload: QuantityPayload,
    
) -> Result<impl Reply, Rejection> {
    let quantity = payload.value();

    let mut conn = pool.get().map_err(|_| warp::reject::reject())?;
    let cart_result_id = get_or_create_cart(&mut conn, user_id)
        .await
        .map_err(|_| warp::reject::reject())?;

    diesel::update(
        cart_items.filter(ci_cart_id.eq(cart_result_id).and(ci_item_id.eq(item_id)))
    )
    .set(crate::schema::cart_items::dsl::quantity.eq(quantity))
    .execute(&mut conn)
    .map_err(|_| warp::reject::reject())?;

    Ok(warp::reply::json(&json!({ "message": "Item quantity updated" })))
}