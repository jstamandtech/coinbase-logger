CREATE TABLE trades (
    id SERIAL PRIMARY KEY,
    trade_id BIGINT NOT NULL,
    sequence BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL,
    product_id CHAR(16) NOT NULL,
    price FLOAT8 NOT NULL,
    last_size FLOAT8 NOT NULL,
    best_bid FLOAT8 NOT NULL,
    best_ask FLOAT8 NOT NULL
)
