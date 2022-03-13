use coinbase_pro_rs::{structs::wsfeed::*, WSFeed, WS_SANDBOX_URL};
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
    let stream = WSFeed::connect(WS_SANDBOX_URL, &["ETH-BTC"], &[ChannelType::Level2])
        .await
        .unwrap();

    stream
        .take(10)
        .try_for_each(|msg| async {
            match msg {
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
