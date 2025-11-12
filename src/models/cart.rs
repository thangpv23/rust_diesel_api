use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::schema::{carts, cart_items};

#[derive(Queryable, Serialize)]
pub struct Cart {
    pub id: i32,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = carts)]
pub struct NewCart {
    pub user_id: i32,
}

#[derive(Queryable, Serialize)]
pub struct CartItem {
    pub id: i32,
    pub cart_id: i32,
    pub item_id: i32,
    pub quantity: i32,
}

#[derive(Insertable)]
#[diesel(table_name = cart_items)]
pub struct NewCartItem {
    pub cart_id: i32,
    pub item_id: i32,
    pub quantity: i32,
}