use chrono::*;
use diesel::prelude::*;
use crate::models::{inventory_model::Inventory, schema::*};

#[derive(Queryable, Identifiable, Selectable, Associations, PartialEq)]
#[diesel(belongs_to(Inventory, foreign_key = inventory_id))]
#[diesel(table_name = restock_history)]
#[diesel(primary_key(restock_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct RestockHistory {
    pub restock_id: i32,
    pub inventory_id: i32,
    pub quantity: i32,
    pub datetime: NaiveDateTime,
}