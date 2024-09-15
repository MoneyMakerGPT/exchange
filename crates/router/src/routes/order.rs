use actix_web::web::{Data, Json};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::time::Instant;
use uuid::Uuid;

use crate::types::app::AppState;

#[derive(Serialize, Deserialize)]
pub struct OrderInput {
    symbol: String,
    side: String,
    order_type: String,
    quantity: Decimal,
    price: Decimal,
    client_id: String,
}

pub async fn execute_order(
    body: Json<OrderInput>,
    app_state: Data<AppState>,
) -> actix_web::HttpResponse {
    let starttime = Instant::now();
    let order_id = Uuid::new_v4();

    let order = body.into_inner();
    let order_json = to_string(&order).unwrap();
    println!("Order: {}", order_json);

    let redis_connection = &app_state.redis_connection;
    let order_data = to_string(&order).unwrap();
    redis_connection.push("orders", order_data).await.unwrap();

    println!("Time: {:?}", starttime.elapsed());

    actix_web::HttpResponse::Ok().finish()
}
