use diesel::{prelude::*, result};
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::{AsyncConnection, RunQueryDsl};

use crate::models::schema::products::dsl::*;
use crate::data::database::*;
use crate::models::product_model::*;

pub async fn get_all_products() -> Vec<Product> {
    let conn = connect_from_pool().await;

    let mut conn = match conn {
        Ok(value) => value,
        Err(_) => panic!("Failed to connect from pool"),
    };

    let res = products
        .select(Product::as_select())
        .load(&mut conn)
        .await;

    return match res {
        Ok(value) => value,
        Err(e) => panic!("Failed to fetch products: {e}"),
    };
}

pub async fn get_product_by_id(id: i32) -> Option<Product> {
    let conn = connect_from_pool().await;

    let mut conn = match conn {
        Ok(value) => value,
        Err(_) => panic!("Failed to connect from pool"),
    };

    let product = products
        .find(id)
        .select(Product::as_select())
        .first(&mut conn)
        .await;

    return match product {
        Ok(value) => Some(value),
        Err(result::Error::NotFound) => None,
        Err(e) => panic!("Error fetching product: {e}"),
    }
}

pub async fn add_product<'a>(new_product: NewProduct<'a>) {
    use crate::models::schema::products;
    
    let pool_conn = connect_from_pool().await;

    let mut conn = match pool_conn {
        Ok(value) => value,
        Err(_) => panic!("Failed to connect from pool"),
    };

    match conn.transaction::<_, result::Error, _>(|connection|
        async move {
            diesel::insert_into(products::table)
                .values(&new_product)
                .execute(connection)
                .await?;

            Ok(())
        }.scope_boxed()
    ).await {
        Ok(_) => {},
        Err(e) => panic!("Database error on adding product: {e}"),
    };
}

pub async fn update_product<'a>(id: i32, update_form: ProductForm<'a>) {
    

    let pool_conn = connect_from_pool().await;

    let mut conn = match pool_conn {
        Ok(value) => value,
        Err(_) => panic!("Failed to connect from pool"),
    };

    match conn.transaction::<_, result::Error, _>(|connection|
        async move {
            let rows_affected: usize = diesel::update(products.find(id))
                .set(&update_form)
                .execute(connection)
                .await?;

            if rows_affected == 0 {
                return Err(result::Error::NotFound);
            }
            Ok(())
        }
        .scope_boxed()
    ).await {
        Ok(_) => {},
        Err(result::Error::NotFound) => println!("Product {id} not found"),
        Err(e) => panic!("Database error when removing product: {e}"),
    };
}

pub async fn remove_product(id: i32) {
    let pool_conn = connect_from_pool().await;

    let mut conn = match pool_conn {
        Ok(value) => value,
        Err(_) => panic!("Failed to connect from pool"),
    };

    match conn.transaction::<_, result::Error, _>(|connection|
        async move {
            let rows_affected: usize = diesel::delete(products.find(id))
                .execute(connection)
                .await?;

            if rows_affected == 0 {
                return Err(result::Error::NotFound);
            }
            Ok(())
        }
        .scope_boxed()
    )
    .await {
        Ok(_) => {},
        Err(result::Error::NotFound) => println!("Product {id} not found"),
        Err(e) => panic!("Database error when removing product: {e}"),
    };
}