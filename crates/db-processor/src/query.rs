use crate::types::{DbTrade, KlineData};
use rust_decimal::Decimal;
use sqlx::{Pool, Postgres};

pub async fn insert_trade(pool: &Pool<Postgres>, trade: DbTrade) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO trades(
          trade_id, market, price, quantity, user_id, other_user_id, order_id, timestamp
      ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(trade.trade_id)
    .bind(trade.market)
    .bind(trade.price)
    .bind(trade.quantity)
    .bind(trade.user_id)
    .bind(trade.other_user_id)
    .bind(trade.order_id)
    .bind(trade.timestamp)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_trades_from_db(
    pool: sqlx::Pool<sqlx::Postgres>,
    market: String,
) -> Result<Vec<DbTrade>, sqlx::Error> {
    let trades = sqlx::query!(
        "SELECT * FROM trades WHERE market = $1 ORDER BY timestamp desc",
        market
    )
    .fetch_all(&pool)
    .await?;

    let trades_vec: Vec<DbTrade> = trades
        .iter()
        .map(|trade| DbTrade {
            trade_id: trade.trade_id.clone(),
            market: trade.market.clone(),
            price: trade.price.to_string().parse::<Decimal>().unwrap(),
            quantity: trade.quantity.to_string().parse::<Decimal>().unwrap(),
            user_id: trade.user_id.clone(),
            other_user_id: trade.other_user_id.clone(),
            order_id: trade.order_id.clone(),
            timestamp: trade.timestamp,
        })
        .collect();

    Ok(trades_vec)
}

pub async fn get_klines_timeseries_data(
    pool: sqlx::Pool<sqlx::Postgres>,
    market: String,
    interval: String,
    start_time: String,
) -> Result<Vec<KlineData>, sqlx::Error> {
    let start_time_int = start_time.parse::<i64>().unwrap();

    let klines = sqlx::query!(
        // The SQL query for generating kline data
        "
        WITH timeseries_data AS (
            SELECT
                date_trunc($1, to_timestamp(timestamp / 1000)) AS bucket,
                price,
                quantity,
                trade_id,
                to_timestamp(timestamp / 1000) AS trade_time,
                ROW_NUMBER() OVER (PARTITION BY date_trunc($1, to_timestamp(timestamp / 1000)) ORDER BY timestamp ASC) AS row_num_asc,
                ROW_NUMBER() OVER (PARTITION BY date_trunc($1, to_timestamp(timestamp / 1000)) ORDER BY timestamp DESC) AS row_num_desc
            FROM trades
            WHERE timestamp >= $2
              AND market = $3
        )
        , aggregated_data AS (
            SELECT
                bucket,
                MAX(price) AS high,
                MIN(price) AS low,
                SUM(quantity) AS volume,
                COUNT(trade_id) AS trades,
                SUM(price * quantity) AS quote_volume,
                MIN(trade_time) AS start_time,
                MAX(trade_time) AS end_time,
                MAX(CASE WHEN row_num_asc = 1 THEN price END) AS open,
                MAX(CASE WHEN row_num_desc = 1 THEN price END) AS close
            FROM timeseries_data
            GROUP BY bucket
        )
        SELECT
            open,
            bucket AS end,
            high,
            low,
            close,
            quote_volume,
            start_time AS start,
            trades,
            volume
        FROM aggregated_data
        ORDER BY bucket ASC
        ",
        interval,      // $1: interval (e.g., 'day', 'week', 'month')
        start_time_int,    // $2: start time for filtering trades
        market         // $3: the market identifier
    )
    .fetch_all(&pool)
    .await?;

    // Map the result set into a vector of KlineData structs
    let kline_data_vec: Vec<KlineData> = klines
        .iter()
        .map(|kline| KlineData {
            open: kline.open.clone().unwrap().to_string(),
            high: kline.high.clone().unwrap().to_string(),
            low: kline.low.clone().unwrap().to_string(),
            close: kline.close.clone().unwrap().to_string(),
            quote_volume: kline.quote_volume.clone().unwrap().to_string(),
            start: kline.start.clone().unwrap().to_string(),
            end: kline.end.clone().unwrap().to_string(),
            trades: kline.trades.clone().unwrap().to_string(),
            volume: kline.volume.clone().unwrap().to_string(),
        })
        .collect();

    Ok(kline_data_vec)
}
