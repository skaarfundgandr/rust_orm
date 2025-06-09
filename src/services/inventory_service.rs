use diesel::{prelude::*, result};
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::{AsyncConnection, RunQueryDsl};

use crate::data::database::*;
use crate::services::product_service::*;
use crate::models::{schema::*, inventory_model::*, product_model::*};

pub async fn get_all_inventory() -> Vec<(Product, Option<Inventory>)>{
    let pool_conn = connect_from_pool().await;

    let mut conn = match pool_conn {
        Ok(value) => value,
        Err(e) => panic!("Error connecting to pool: {e}"),
    };

    let all_products: Vec<Product> = get_all_products().await;

    let inventory: Vec<Inventory> = match Inventory::belonging_to(&all_products)
        .select(Inventory::as_select())
        .load(&mut conn)
        .await {
            Ok(value) => value,
            Err(e) => panic!("Error fetching inventory: {e}"),
        };

    return inventory
        .grouped_by(&all_products)
        .into_iter()
        .zip(all_products)
        .map(|(inv, product)| {
            match inv.len() {
                0 => (product, None),
                1 => (product, Some(inv.into_iter().next().unwrap())),
                _ => panic!("Multiple inventories for product {} found", product.product_id),
            }
        })
        .collect::<Vec<(Product, Option<Inventory>)>>();
}

pub async fn add_inventory<'a>(new_inventory: NewInventory<'a>) {
    let pool_conn = connect_from_pool().await;

    let mut conn = match pool_conn {
        Ok(value) => value,
        Err(e) => panic!("Error connecting to pool: {e}"),
    };

    match conn.transaction::<_, result::Error, _>(|connection|
        async move {
            diesel::insert_into(inventory::table)
                .values(&new_inventory)
                .execute(connection)
                .await?;

            Ok(())
        }.scope_boxed()
    ).await {
        Ok(_) => {},
        Err(e) => panic!("Database error on adding product to inventory: {e}"),
    };
}