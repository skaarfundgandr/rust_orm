use diesel::prelude::*;
use crate::models::{product_model::Product, schema::*};

#[derive(Queryable, Identifiable, Selectable, Associations, PartialEq, Debug)]
#[diesel(belongs_to(Product, foreign_key = product_id))]
#[diesel(table_name = inventory)]
#[diesel(primary_key(inventory_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Inventory {
    pub inventory_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Insertable)]
#[diesel(table_name = inventory)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewInventory<'a> {
    pub product_id: &'a i32,
    pub quantity: &'a i32,
}

#[derive(AsChangeset)]
#[diesel(table_name = inventory)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct InventoryForm<'a> {
    pub product_id: Option<&'a i32>,
    pub quantity: Option<&'a i32>,
}