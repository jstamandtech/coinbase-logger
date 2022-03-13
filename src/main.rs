use coinbase_pro_rs::{structs::wsfeed::*, WSFeed, WS_SANDBOX_URL};
use futures::{StreamExt, TryStreamExt};


#[tokio::main]
async fn main() {
    let stream = WSFeed::connect(WS_SANDBOX_URL, &["ETH-BTC"], &[ChannelType::Heartbeat])
        .await
        .unwrap();

    stream
        .take(10)
        .try_for_each(|msg| async {
            match msg {
                Message::Heartbeat {
                    sequence,
                    last_trade_id,
                    time,
                    ..
                } => println!("{}: seq:{} id{}", time, sequence, last_trade_id),
                Message::Error { message } => println!("Error: {}", message),
                Message::InternalError(_) => panic!("internal_error"),
                other => println!("{:?}", other),
            };
            Ok(())
        })
        .await
        .expect("stream fail");

}
