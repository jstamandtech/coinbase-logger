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
use coinbase_pro_rs::{structs::wsfeed::*, WSFeed, WS_SANDBOX_URL, WS_URL};
use futures::{StreamExt, TryStreamExt};

#[tokio::main]
async fn main() {
    let conn = establish_connection();

    let stream = WSFeed::connect(WS_URL, &["status"], )

}