#[derive(Queryable)]
pub struct Activity {
    pub id: i32,
    pub user_id: String,
    pub activity_date: NaiveDate,
    pub total_steps: i32,
    pub total_distance: f32,
    pub calories: i32,
}

use std::time::SystemTime;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::data_types::{PgDate, PgTimestamp};
use super::schema::daily_activity;

#[derive(Insertable)]
#[table_name = "daily_activity"]
pub struct NewActivity<'a> {
    pub user_id: &'a String,
    pub activity_date: &'a NaiveDate,
    pub total_steps: &'a i32,
    pub total_distance: &'a f32,
    pub calories: &'a i32,
}