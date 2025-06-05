use diesel::prelude::*;
use crate::models::schema::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = products)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Products {
    pub product_id: i32,
    pub product_name: String,
    pub product_desc: Option<String>,
    pub product_category: String,
    pub product_price: f32
}