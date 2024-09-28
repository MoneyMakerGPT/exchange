use actix_web::web::{Data, Json};

use serde_json::{from_str, to_string};
use std::time::Instant;
use uuid::Uuid;

use crate::types::{
    app::AppState,
    routes::{CancelOrderInput, CreateOrderInput, GetOpenOrdersInput, OrderRequests},
};

pub async fn execute_order(
    body: Json<CreateOrderInput>,
    app_state: Data<AppState>,
) -> actix_web::HttpResponse {
    let starttime = Instant::now();
    let order_id = Uuid::new_v4();

    let order = body.into_inner();
    let create_order_request = OrderRequests::CreateOrder(order);
    let create_order_data = to_string(&create_order_request).unwrap();
    println!("Create Order: {}", create_order_data);

    let redis_connection = &app_state.redis_connection;
    redis_connection
        .push("orders", create_order_data)
        .await
        .unwrap();

    println!("Time: {:?}", starttime.elapsed());
    actix_web::HttpResponse::Ok().finish()
}

pub async fn cancel_order(
    body: Json<CancelOrderInput>,
    app_state: Data<AppState>,
) -> actix_web::HttpResponse {
    let starttime = Instant::now();
    let order_id = Uuid::new_v4();

    let order = body.into_inner();
    let cancel_order_request = OrderRequests::CancelOrder(order);
    let cancel_order_data = to_string(&cancel_order_request).unwrap();
    println!("Cancel Order: {}", cancel_order_data);

    let redis_connection = &app_state.redis_connection;
    redis_connection
        .push("orders", cancel_order_data)
        .await
        .unwrap();

    println!("Time: {:?}", starttime.elapsed());

    actix_web::HttpResponse::Ok().finish()
}

pub async fn get_open_orders(
    body: Json<GetOpenOrdersInput>,
    app_state: Data<AppState>,
) -> actix_web::HttpResponse {
    let starttime = Instant::now();
    let order_id = Uuid::new_v4();

    let order = body.into_inner();
    let get_open_orders_request = OrderRequests::GetOpenOrders(order);
    let get_open_orders_data = to_string(&get_open_orders_request).unwrap();
    println!("Get Open Orders: {}", get_open_orders_data);

    let redis_connection = &app_state.redis_connection;
    redis_connection
        .push("orders", get_open_orders_data)
        .await
        .unwrap();

    println!("Time: {:?}", starttime.elapsed());

    actix_web::HttpResponse::Ok().finish()
}
