use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;  
use crate::schema::items;  

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = items)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = items)]
pub struct NewItem {
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
}
