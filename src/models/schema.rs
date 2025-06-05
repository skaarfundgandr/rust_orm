// @generated automatically by Diesel CLI.

diesel::table! {
    products (product_id) {
        product_id -> Integer,
        #[max_length = 50]
        product_name -> Varchar,
        product_desc -> Nullable<Text>,
        product_category -> Text,
        product_price -> Float,
    }
}
