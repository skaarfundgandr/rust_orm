use rust_orm::services::product_service::get_products;

#[tokio::main]
async fn main() {
    let products = get_products().await;

    println!("Displaying {} products", products.len());

    for product in products {
        println!("Product Name: {}", product.product_name);
    }
}