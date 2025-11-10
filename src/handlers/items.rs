// Items handlers: CRUD
use warp::{Reply, Rejection};
use crate::db::DbPool;
use crate::models::item::Item;
use crate::models::item::NewItem;
use diesel::prelude::*;
use serde::Deserialize;

pub async fn create_item(pool: DbPool, body: NewItem) -> Result<impl Reply, Rejection> {
    use crate::schema::items::dsl::*;
    let mut conn = pool.get().map_err(|_| warp::reject::reject())?;

    let new_item = NewItem {
        name: body.name,
        description: body.description,
        price: body.price,
        stock: body.stock,
    };

    diesel::insert_into(items)
        .values(&new_item)
        .execute(&mut conn)
        .map_err(|_| warp::reject::reject())?;

    Ok(warp::reply::json(&"Item created"))
}

pub async fn read_item(pool: DbPool, id: i32) -> Result<impl Reply, Rejection> {
    use crate::schema::items::dsl::*;
    let mut conn = pool.get().map_err(|_| warp::reject::reject())?;

    let result = items
        .find(id)
        .first::<Item>(&mut conn)
        .map_err(|_| warp::reject::reject())?;

    Ok(warp::reply::json(&result))
}

pub async fn list_items(_user_id: i32, pool: DbPool) -> Result<impl Reply, Rejection> {
    use crate::schema::items::dsl::*;
    let mut conn = pool.get().map_err(|_| warp::reject::reject())?;

    let results = items.load::<Item>(&mut conn).map_err(|_| warp::reject::reject())?;

    Ok(warp::reply::json(&results))
}

pub async fn update_item(pool: DbPool, id: i32, body: NewItem) -> Result<impl Reply, Rejection> {
    use crate::schema::items::dsl::*;
    let mut conn = pool.get().map_err(|_| warp::reject::reject())?;

    diesel::update(items.find(id))
        .set((
            name.eq(body.name),
            description.eq(body.description),
            price.eq(body.price),
            stock.eq(body.stock),
        ))
        .execute(&mut conn)
        .map_err(|_| warp::reject::reject())?;

    Ok(warp::reply::json(&"Item updated"))
}

pub async fn delete_item(pool: DbPool, id: i32) -> Result<impl Reply, Rejection> {
    use crate::schema::items::dsl::*;
    let mut conn = pool.get().map_err(|_| warp::reject::reject())?;

    diesel::delete(items.find(id))
        .execute(&mut conn)
        .map_err(|_| warp::reject::reject())?;

    Ok(warp::reply::json(&"Item deleted"))
}
