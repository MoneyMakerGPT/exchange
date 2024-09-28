pub mod engine;
pub mod types;

use engine::engine::Engine;
use fred::prelude::RedisValue;
use redis::{RedisManager, RedisQueues};
use serde_json::from_str;
use types::engine::OrderRequests;

#[tokio::main]
async fn main() {
    let redis_connection = RedisManager::new().await.unwrap();
    println!("Redis connected!");

    let mut engine = Engine::new();
    engine.init_engine();
    engine.init_user_balance("random_id");

    loop {
        match redis_connection
            .pop(RedisQueues::ORDERS.to_string().as_str(), Some(1))
            .await
        {
            Ok(data) => {
                if data.len() > 0 {
                    let order_to_process = &data[0];

                    // Convert the RedisValue to a string
                    let order_data = match order_to_process {
                        RedisValue::String(s) => s.to_string(),
                        // BYTES TYPE - NOT NEEDED FOR NOW - check if this can make it faster
                        RedisValue::Bytes(b) => {
                            String::from_utf8(b.to_vec()).unwrap_or_else(|_| "".to_string())
                        }
                        _ => {
                            println!("Unexpected Redis value type");
                            continue;
                        }
                    };

                    // Now you can deserialize it using serde_json
                    match from_str::<OrderRequests>(&order_data) {
                        Ok(order) => match order {
                            OrderRequests::CreateOrder(order) => {
                                println!("Create Order: {:?}", order);
                                let pubsub_id = order.pubsub_id.unwrap().to_string();
                                let pubsub_id_ref = pubsub_id.as_str();

                                let create_order_result = engine.create_order(order);

                                match create_order_result {
                                    Ok(()) => {
                                        let _ = redis_connection
                                            .publish(pubsub_id_ref, String::from("Created Order"))
                                            .await;

                                        println!("Successfully placed order!")
                                    }
                                    Err(str) => {
                                        println!("Order creation failed - {}", str)
                                    }
                                }
                            }

                            OrderRequests::CancelOrder(cancel_order) => {
                                println!("Cancel Order: {:?}", cancel_order);
                                let pubsub_id = cancel_order.pubsub_id.unwrap().to_string();
                                let pubsub_id_ref = pubsub_id.as_str();

                                let cancel_order_result = engine.cancel_order(cancel_order);

                                match cancel_order_result {
                                    Ok(()) => {
                                        let _ = redis_connection
                                            .publish(pubsub_id_ref, String::from("Cancelled Order"))
                                            .await;
                                        println!("Successfully cancelled order!")
                                    }
                                    Err(str) => {
                                        println!("Order cancellation failed - {}", str)
                                    }
                                }
                            }

                            OrderRequests::GetOpenOrders(open_orders) => {
                                println!("Open Order: {:?}", open_orders);
                                let pubsub_id = open_orders.pubsub_id.unwrap().to_string();
                                let pubsub_id_ref = pubsub_id.as_str();

                                let open_orders_vec = engine.get_open_orders(open_orders);

                                let _ = redis_connection
                                    .publish(pubsub_id_ref, String::from("Open Orders Retrieved"))
                                    .await;
                                println!(
                                    "Successfully retrieved open orders! {:?}",
                                    open_orders_vec
                                );
                            }
                        },
                        Err(err) => {
                            println!("Failed to deserialize order request: {:?}", err);
                        }
                    }
                }
            }

            Err(error) => {
                println!("Error popping from Redis: {:?}", error);
            }
        }
    }
}
