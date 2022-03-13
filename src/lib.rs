#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;
use dotenv::dotenv;
use self::models::{Trade, NewTrade};
use coinbase_pro_rs::structs::wsfeed::Ticker;
use diesel::{PgConnection, RunQueryDsl, Connection};


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))

}


pub fn create_trade<'a>(conn: &PgConnection, ticker: &Ticker) {
    use schema::trades;

    match ticker {
        Ticker::Full {
            trade_id,
            sequence,
            time,
            product_id,
            price,
            side,
            last_size,
            best_bid,
            best_ask
        } => {
            let new_trade: NewTrade = NewTrade{
                trade_id: *trade_id as i64,
                sequence: *sequence as i64,
                time: time.naive_utc(),
                product_id: &*product_id,
                price: *price,
                last_size: *last_size,
                best_bid: *best_bid,
                best_ask: *best_ask
            };

            diesel::insert_into(trades::table)
                .values(&new_trade)
                .get_result::<Trade>(conn)
                .expect("Error saving trade");

        },
        Ticker::Empty { .. } => {}
    }
}