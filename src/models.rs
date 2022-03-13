//great resource on mapping PG, Diesel, and Rus Types
// https://kotiri.com/2018/01/31/postgresql-diesel-rust-types.html

use crate::schema::trades;
use chrono::{DateTime, TimeZone, Utc, NaiveDateTime};

#[derive(Queryable)]
pub struct Trade{
    pub id: i32,
    pub trade_id: i64,
    pub sequence: i64,
    pub time: NaiveDateTime,
    pub product_id: String,
    pub price: f64,
    pub last_size: f64,
    pub best_bid: f64,
    pub best_ask: f64,
}


#[derive(Insertable)]
#[table_name="trades"]
pub struct NewTrade<'a> {
    pub trade_id: i64,
    pub sequence: i64,
    pub time: NaiveDateTime,
    pub product_id: &'a str,
    pub price: f64,
    pub last_size: f64,
    pub best_bid: f64,
    pub best_ask: f64,
}
