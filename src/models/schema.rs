// @generated automatically by Diesel CLI.

diesel::table! {
    inventory (inventory_id) {
        inventory_id -> Integer,
        product_id -> Integer,
        quantity -> Integer,
    }
}

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

diesel::joinable!(inventory -> products (product_id));

diesel::allow_tables_to_appear_in_same_query!(
    inventory,
    products,
);
