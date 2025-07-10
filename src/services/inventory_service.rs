use diesel::{prelude::*, result};
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::{AsyncConnection, RunQueryDsl};

use crate::data::database::*;
use crate::services::product_service::*;
use crate::models::{schema::*, inventory_model::*, product_model::*};

pub async fn get_all_inventory() -> Vec<(Product, Option<Inventory>)> {
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

pub async fn get_inventory_by_product_id(product_id: i32) -> Result<Option<Inventory>, result::Error> {
    let pool_conn = connect_from_pool().await;
    let mut conn = match pool_conn {
        Ok(value) => value,
        Err(e) => panic!("Error connecting to pool: {e}")
    };

    let inventory = inventory::table
        .filter(inventory::product_id.eq(product_id))
        .first::<Inventory>(&mut conn)
        .await;

    return match inventory {
        Ok(value) => Ok(Some(value)),
        Err(result::Error::NotFound) => Ok(None),
        Err(e) => Err(e)
    }
}

pub async fn get_inventory_by_id(inventory_id: i32) -> Result<Option<Inventory>, result::Error> {
    let pool_conn = connect_from_pool().await;

    let mut conn = match pool_conn {
        Ok(value) => value,
        Err(e) => panic!("Error connecting to pool: {e}")
    };

    let inventory = inventory::table
        .filter(inventory::inventory_id.eq(inventory_id))
        .first::<Inventory>(&mut conn)
        .await;

    return match inventory {
        Ok(value) => Ok(Some(value)),
        Err(result::Error::NotFound) => Ok(None),
        Err(e) => Err(e)
    };
}

pub async fn add_inventory<'a>(new_inventory: NewInventory<'a>) -> Result<(), result::Error>{
    let pool_conn = connect_from_pool().await;

    let mut conn = match pool_conn {
        Ok(value) => value,
        Err(e) => panic!("Error connecting to pool: {e}"),
    };

    return match conn.transaction::<_, result::Error, _>(|connection|
        async move {
            diesel::insert_into(inventory::table)
                .values(&new_inventory)
                .execute(connection)
                .await?;

            Ok(())
        }.scope_boxed()
    ).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
}

pub async fn update_inventory<'a>(id: i32, inventory_form: InventoryForm<'a>) -> Result<(), result::Error> {
    use crate::models::schema::inventory::dsl::*;

    let pool_conn = connect_from_pool().await;

    let mut conn = match pool_conn {
        Ok(value) => value,
        Err(e) => panic!("Error connecting to pool: {e}"),
    };

    return match conn.transaction::<_, result::Error, _>(|connection|
        async move {
            let rows_affected: usize = diesel::update(inventory.find(id))
                .set(inventory_form)
                .execute(connection)
                .await?;

            if rows_affected == 0 {
                return Err(result::Error::NotFound);
            }

            Ok(())
        }
        .scope_boxed()
    ).await {
        Ok(_) => Ok(()),
        Err(result::Error::NotFound) => Err(result::Error::NotFound),
        Err(e) => Err(e),
    };
}

pub async fn remove_inventory(id: i32) -> Result<(), result::Error> {
    use crate::models::schema::inventory::dsl::*;

    let pool_conn = connect_from_pool().await;

    let mut conn = match pool_conn {
        Ok(value) => value,
        Err(e) => panic!("Error connecting to pool: {e}"),
    };

    return match conn.transaction::<_, result::Error, _>(|connection|
        async move {
            let rows_affected: usize = diesel::delete(inventory.find(id))
                .execute(connection)
                .await?;

            if rows_affected == 0 {
                return Err(result::Error::NotFound);
            }

            Ok(())
        }.scope_boxed()
    ).await {
        Ok(_) => Ok(()),
        Err(result::Error::NotFound) => Err(result::Error::NotFound),
        Err(e) => Err(e),
    };
}
// WARN: DESTRUCTIVE
pub async fn clear_inventory() -> Result<(), result::Error> {
    use crate::models::schema::inventory::dsl::*;

    let pool_conn = connect_from_pool().await;

    let mut conn = match pool_conn {
        Ok(value) => value,
        Err(e) => panic!("Error connecting to pool: {e}"),
    };

    return match conn.transaction::<_, result::Error, _>(|connection|
        async move {
            diesel::delete(inventory)
                .execute(connection)
                .await?;
            Ok(())
        }.scope_boxed()
    ).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
}