pub mod engine;
use engine::engine::{CreateOrder, Engine};
use fred::prelude::RedisValue;
use redis::RedisManager;
use serde_json::from_str;

#[tokio::main]
async fn main() {
    let redis_connection = RedisManager::new().await.unwrap();
    println!("Redis connected!");

    let mut engine = Engine::new();
    engine.init_engine();
    engine.init_user_balance("random_id");

    loop {
        match redis_connection.pop("orders", Some(1)).await {
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
                    match from_str::<CreateOrder>(&order_data) {
                        Ok(order) => {
                            println!("Order: {:?}", order);
                            let order_result = engine.create_order(order);

                            match order_result {
                                Ok(()) => {
                                    println!("Successfully placed order!")
                                }
                                Err(str) => {
                                    println!("Order creation failed - {}", str)
                                }
                            }
                        }
                        Err(err) => {
                            println!("Failed to deserialize order: {:?}", err);
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
