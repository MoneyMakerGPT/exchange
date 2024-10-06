use crate::types::DbTrade;
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
