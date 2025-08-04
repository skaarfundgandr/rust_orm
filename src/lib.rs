pub mod data;
pub mod services;
pub mod models;

#[cfg(test)]
mod tests {
    use crate::services::inventory_service::clear_inventory;

    use super::*;

    #[tokio::test]
    async fn test_conn_async() {
        let _conn: diesel_async::AsyncMysqlConnection = data::database::connect_to_db_async().await;

        println!("Test passed");
    }
    #[tokio::test]
    async fn test_pooled_conn() {
        let mut _conn = data::database::connect_from_pool().await;

        assert!(_conn.is_ok());

        println!("Test passed")
    }
    #[tokio::test]
    #[serial_test::serial]
    async fn test_product_service() {
        use models::product_model::*;
        use services::product_service::*;

        // Clear products and inventory tables before running tests
        clear_inventory().await.expect("error clearing inventory table");
        clear_products().await.expect("error clearing products table");

        let new_product: NewProduct<'_> = NewProduct {
            product_name: "test",
            product_desc: Some("testdesc"),
            product_category: "testcat",
            product_price: &69.0,
        };

        add_product(new_product).await.expect("error adding product");

        let all_products: Vec<Product> = get_all_products().await;

        let num_products: usize = all_products.len();

        assert!(num_products == 1);

        let first_product: &Product = all_products.iter().next().unwrap();

        assert!(first_product.product_name == "test");

        update_product(first_product.product_id, ProductForm { 
            product_name: Some("updated_prod"), 
            product_desc: None, 
            product_category: None, 
            product_price: None 
        }).await;

        let updated_product: Product = get_product_by_id(first_product.product_id).await.expect("err fetching product");

        assert!(updated_product.product_name == "updated_prod");

        remove_product(updated_product.product_id).await;

        let all_products: Vec<Product> = get_all_products().await;

        assert!(all_products.len() == 0);
        println!("Test passed!");
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_inventory_service() {
        use crate::models::product_model::*;
        use crate::models::inventory_model::*;
        use crate::services::product_service::*;
        use crate::services::inventory_service::*;
        // Clear inventory and products table before running tests
        clear_inventory().await.expect("err clearing inventory");
        clear_products().await.expect("err clearing products");

        let new_product: NewProduct<'_> = NewProduct { 
            product_name: "testprod",
            product_desc: Some("desc"), 
            product_category: "testcategory", 
            product_price: &10.99,
        };

        add_product(new_product).await.expect("err adding product");

        let inserted_product: Product = get_product_by_name("testprod").await.expect("err fetching product");

        let new_inventory: NewInventory<'_> = NewInventory {
            product_id: &inserted_product.product_id,
            quantity: &5,   
        };

        add_inventory(new_inventory).await.expect("err adding inventory");

        let inventory: Inventory = match get_inventory_by_product_id(inserted_product.product_id)
            .await
            .expect("err fetching inventory")
            {
                Some(value) => value,
                None => panic!("err fetching inventory")
            };
        assert_eq!(inventory.quantity, 5);

        let all_inventory: Vec<(Product, Option<Inventory>)> = get_all_inventory().await;

        assert_eq!(all_inventory.len(), 1);

        if let Some((product, inventory)) = all_inventory.first() {
            assert_eq!(product.product_name, "testprod");
            match inventory {
                Some(value) => assert_eq!(value.quantity, 5),
                None => panic!("err fetching inventory")  
            };
        } else {
            panic!("err fetching all inventory");
        }

        let inv_form: InventoryForm<'_> = InventoryForm {
            product_id: None,
            quantity: Some(&3)
        };

        update_inventory(inventory.inventory_id, inv_form)
            .await
            .expect("err updating inventory");

        let updated_inventory = match get_inventory_by_id(inventory.inventory_id)
            .await
            .expect("err fetching updated inventory")
            {
                Some(value) => value,
                None => panic!("err fetching updated inventory")
            };

        assert_eq!(updated_inventory.quantity, 3);

        remove_inventory(updated_inventory.inventory_id)
            .await
            .expect("err removing inventory");

        let removed_inventory: Option<Inventory> = get_inventory_by_id(inventory.inventory_id)
            .await
            .expect("err fetching removed inventory");

        assert_eq!(removed_inventory, None);
        println!("Test passed!");
    }
}
