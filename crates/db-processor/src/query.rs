use crate::types::DbTrade;
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

pub async fn get_trade(pool: sqlx::Pool<sqlx::Postgres>) -> Result<String, sqlx::Error> {
    let trade = sqlx::query!("SELECT * FROM trades")
        .fetch_one(&pool)
        .await?;

    Ok(trade.order_id)
}
