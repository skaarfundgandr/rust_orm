use diesel::{insert_into, prelude::*, result};
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::{AsyncConnection, RunQueryDsl};

use crate::data::database::*;
use crate::models::product_model::*;
use crate::models::schema::products;

pub async fn get_products() -> Vec<Product> {
    use crate::models::schema::products::dsl::*;

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
        Err(_) => panic!("Failed to fetch products"),
    };
}

pub async fn add_product<'a>(new_product: NewProduct<'a>) {
    let pool_conn = connect_from_pool().await;

    let mut conn = match pool_conn {
        Ok(value) => value,
        Err(_) => panic!("Failed to connect from pool"),
    };

    conn.transaction::<_, result::Error, _>(|connection|
        async move {
            insert_into(products::table)
                .values(&new_product)
                .execute(connection)
                .await?;

            products::table
                .order(products::product_id.desc())
                .select(Product::as_select())
                .first(connection)
                .await?;
            Ok(())
    }.scope_boxed()).await
    .expect("Transaction failed");
}

pub async fn update_product<'a>(id: i32, update_form: ProductForm<'a>) {
    use crate::models::schema::products::dsl::*;
    
    let pool_conn = connect_from_pool().await;

    let mut conn = match pool_conn {
        Ok(value) => value,
        Err(_) => panic!("Failed to connect from pool"),
    };

    conn.transaction::<_, result::Error, _>(|connection|
        async move {
            diesel::update(products.find(id))
                .set(&update_form)
                .execute(connection)
                .await?;
            Ok(())
        }
        .scope_boxed()
    ).await
    .expect("Error updating product")
}