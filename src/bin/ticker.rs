#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use coinbase_api::{establish_connection, create_trade};

use coinbase_pro_rs::{structs::wsfeed::*, WSFeed, WS_SANDBOX_URL, WS_URL};
use futures::{StreamExt, TryStreamExt};


fn print_message(msg: Level2) {
    match msg {
        Level2::Snapshot {
            product_id,
            ..
        } => {
            println!{"Snapshot - product id: {}", product_id}
        },
        Level2::L2update {
            product_id,
            ..
        } => {
            println!{"Update - product id: {}", product_id}
        }
    }
}

#[tokio::main]
async fn main() {
    let product_ids = &["BTC-USD", "ETH-USD", "USDT-USD", "ADA-USD", "DOGE-USD", "XTZ-USD",
        "SHIB-USD", "DOT-USD", "XTZ-USD", "AVAX-USD", "SHIB-USD", "SOL-USD", "ICP-USD", "LTC-USD",
        "BCH-USD", "EOS-USD", "DASH-USD", "OXT-USD", "MKR-USD", "XLM-USD", "ENJ-USD", "ATOM-USD",
        "AXS-USD", "ETC-USD", "OMG-USD", "UST-USD", "AMP-USD", "ZEC-USD", "LINK-USD", "BAT-USD",
        "CHZ-USD", "STX-USD", "CRO-USD", "PAX-USD", "QNT-USD", "REP-USD", "ZRX-USD", "ALGO-USD",
        "ZEN-USD", "DAI-USD", "LPT-USD", "1INCH-USD", "MANA-USD", "LOOM-USD", "IMX-USD", "KNC-USD",
        "CVC-USD", "PERP-USD", "VGX-USD", "ENS-USD", "DNT-USD", "DESO-USD", "SPELL-USD", "COMP-USD",
        ];

    //
    let ws_feed_url =  dotenv::var("WS_FEED_URL").expect("WS_FEED_URL must be set");

    //first connect to the database
    let conn = establish_connection();

    let stream = WSFeed::connect(&ws_feed_url, product_ids, &[ChannelType::Ticker])
        .await
        .unwrap();

    stream
        //.take(1000)
        .try_for_each(|msg| async {
            match msg {
                Message::Ticker(ticker) => {
                    match ticker {
                        Ticker::Full {
                            trade_id,
                            sequence,
                            time,
                            ref product_id,
                            price,
                            ..
                        } => {
                            println!("Trade ID: {}", trade_id);
                            println!("Sequence: {}", sequence);
                            println!("Time: {}", time);
                            println!("Product ID: {}", product_id);
                            println!("price: {}", price);
                            create_trade(&conn, &ticker);
                        },
                        _ => {}
                    }
                },

                Message::Level2 (l2) => {print_message(l2);},
                // Message::Heartbeat {
                //     sequence,
                //     last_trade_id,
                //     time,
                //     ..
                // } => println!("{}: seq:{} id{}", time, sequence, last_trade_id),
                Message::Error { message } => println!("Error: {}", message),
                Message::InternalError(_) => panic!("internal_error"),
                other => println!("{:?}", other),
            }
            Ok(())
        })
        .await
        .expect("stream fail");
}
