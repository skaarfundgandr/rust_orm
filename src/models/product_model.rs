use diesel::prelude::*;
use crate::models::schema::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = products)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Product {
    pub product_id: i32,
    pub product_name: String,
    pub product_desc: Option<String>,
    pub product_category: String,
    pub product_price: f32
}

#[derive(Insertable)]
#[diesel(table_name = products)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewProduct<'a> {
    pub product_name: &'a str,
    pub product_desc: Option<&'a str>,
    pub product_category: &'a str,
    pub product_price: &'a f32,
}