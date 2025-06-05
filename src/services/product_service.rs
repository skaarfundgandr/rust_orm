use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::data::database::*;
use crate::models::product_model::*;

pub async fn get_products() -> Vec<Products> {
    use crate::models::schema::products::dsl::*;

    let conn = connect_from_pool().await;

    let mut conn = match conn {
        Ok(value) => value,
        Err(_) => panic!("Failed to connect from pool"),
    };

    let res = products
        .select(Products::as_select())
        .load(&mut conn)
        .await;

    return match res {
        Ok(value) => value,
        Err(_) => panic!("Failed to fetch products"),
    };
}