pub mod engine;
pub mod order;
pub mod types;

use engine::engine::Engine;
use order::handle_order;
use redis::{RedisManager, RedisQueues};

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
                    handle_order(data, &redis_connection, &mut engine).await;
                }
            }
            Err(error) => {
                println!("Error popping from Redis: {:?}", error);
            }
        }
    }
}
