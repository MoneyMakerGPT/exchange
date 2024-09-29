use db_processor::handle_db_updates;
use redis::{RedisManager, RedisQueues};
pub mod types;

#[tokio::main]
async fn main() {
    let redis_connection = RedisManager::new().await.unwrap();
    println!("Redis connected!");

    loop {
        match redis_connection
            .pop(RedisQueues::DATABASE.to_string().as_str(), Some(1))
            .await
        {
            Ok(data) => {
                if data.len() > 0 {
                    handle_db_updates(data).await;
                }
            }
            Err(error) => {
                println!("Error popping from Redis: {:?}", error);
            }
        }
    }
}
