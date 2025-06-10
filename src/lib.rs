pub mod data;
pub mod services;
pub mod models;

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_conn_async() {
        let _conn = data::database::connect_to_db_async().await;
    }
    #[tokio::test]
    async fn test_pooled_conn() {
        let mut _conn = data::database::connect_from_pool().await;
    }
    #[tokio::test]
    async fn test_product_service() {
        use models::product_model::*;
        use services::product_service::*;

        let new_product = NewProduct {
            product_name: "test",
            product_desc: Some("testdesc"),
            product_category: "testcat",
            product_price: &69.0,
        };

        add_product(new_product).await;

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
    }
}
