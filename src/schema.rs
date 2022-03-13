table! {
    trades (id) {
        id -> Int4,
        trade_id -> Int8,
        sequence -> Int8,
        time -> Timestamptz,
        product_id -> Bpchar,
        price -> Float8,
        last_size -> Float8,
        best_bid -> Float8,
        best_ask -> Float8,
    }
}
