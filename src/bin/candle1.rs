#[macro_use]
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;

use coinbase_api::{establish_connection};

use coinbase_api::models::*;

//use coinbase_api::schema::trades::dsl::trades;
use coinbase_api::schema::trades;
use diesel::{RunQueryDsl, ExpressionMethods};

use coinbase_api::models::Trade;
use diesel::query_dsl::methods::OrderDsl;

fn main() {

    let conn = establish_connection();

    let results = trades::dsl::trades
        .order(trades::sequence.asc())
        .load::<Trade>(&conn).expect("Error loading trades");




    let a: i32 = 1;


}