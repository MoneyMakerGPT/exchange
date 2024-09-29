use std::fmt::Error;
use uuid::Uuid;

use fred::{clients::SubscriberClient, prelude::*};

pub enum RedisQueues {
    ORDERS,
    USERS,
}

impl ToString for RedisQueues {
    fn to_string(&self) -> String {
        match self {
            RedisQueues::ORDERS => "orders".to_string(),
            RedisQueues::USERS => "users".to_string(),
        }
    }
}

pub struct RedisManager {
    client: RedisClient,
    publisher: RedisClient,
    subscriber: SubscriberClient,
}

impl RedisManager {
    pub async fn new() -> Result<Self, RedisError> {
        let client = Builder::default_centralized().build()?;
        let publisher = Builder::default_centralized().build()?;
        let subscriber = Builder::default_centralized().build_subscriber_client()?;

        client.init().await?;
        publisher.init().await?;
        subscriber.init().await?;

        Ok(Self {
            client,
            publisher,
            subscriber,
        })
    }

    pub async fn push(&self, key: &str, value: String) -> Result<(), RedisError> {
        self.client.lpush(key, value).await
    }

    pub async fn pop(
        &self,
        key: &str,
        count: Option<usize>,
    ) -> Result<Vec<RedisValue>, RedisError> {
        self.client.rpop(key, count).await
    }

    pub async fn publish(&self, channel: &str, value: String) -> Result<(), RedisError> {
        self.publisher.publish(channel, value).await
    }

    pub async fn subscribe(&self, channel: &str) -> Result<(), RedisError> {
        self.subscriber.subscribe(channel).await
    }

    pub async fn unsubscribe(&self, channel: &str) -> Result<(), RedisError> {
        self.subscriber.unsubscribe(channel).await
    }

    pub async fn push_and_wait_for_subscriber(
        &self,
        key: String,
        value: String,
        channel: Uuid,
    ) -> Result<(), Error> {
        self.push(key.as_str(), value).await.map_err(|e| {
            println!("Couldn't push into queue - {}", e);
            Error
        })?;

        let channel = channel.to_string();
        let channel_ref = channel.as_str();
        self.subscribe(channel_ref).await.map_err(|e| {
            println!("Failed to subscribe to channel - {}", e);
            Error
        })?;

        let mut message_stream = self.subscriber.message_rx();
        if let Ok(message) = message_stream.recv().await {
            println!("Recv {:?} on channel {}", message.value, message.channel);

            let _ = self.unsubscribe(channel_ref).await;
            return Ok(());
        }

        Ok(())
    }
}

// // Singleton Implementation

// add dependency - once_cell = "1.12"
// use once_cell::sync::Lazy;
// use std::{
//     fmt::Error,
//     sync::{Mutex, MutexGuard},
// };
// // Define a Lazy static instance of RedisManager
// static REDIS_MANAGER: Lazy<Mutex<Option<RedisManager>>> = Lazy::new(|| Mutex::new(None));

// // Function to access the RedisManager singleton instance
// pub async fn get_redis_manager() -> Result<MutexGuard<'static, Option<RedisManager>>, RedisError> {
//     let mut redis_manager = REDIS_MANAGER.lock().unwrap(); // Lock the mutex to ensure thread safety

//     if redis_manager.is_none() {
//         // If the instance has not been created yet, initialize it
//         let instance = RedisManager::new().await?;
//         *redis_manager = Some(instance);
//     }

//     Ok(redis_manager)
// }

// Example usage:

// ----------------------------------------
// QUEUE
// ----------------------------------------

// let client = Builder::default_centralized().build()?;
// client.init().await?;

// // Ensure the key is cleared first to avoid type conflicts
// client.del("foo").await?;
// client.lpush("foo", 111).await?;

// let value: Vec<RedisValue> = client.rpop("foo", Some(2)).await?;
// println!("RPOP Value: {:?}", value);
// client.quit().await?;
// Ok(())

// ----------------------------------------
// PUBSUB
// ----------------------------------------
// let publisher_client = Builder::default_centralized().build()?;
// let subscriber_client = Builder::default_centralized().build_subscriber_client()?;
// publisher_client.init().await?;
// subscriber_client.init().await?;

// // Subscribe to the "foo" channel
// subscriber_client.subscribe("foo").await?;

// // or use `message_rx()` to use the underlying `BroadcastReceiver` directly without spawning a new task
// let _message_task = subscriber_client.on_message(|message| {
//     println!("{}: {}", message.channel, message.value.convert::<i64>()?);
//     Ok::<_, RedisError>(())
// });

// for idx in 0 .. 50 {
//     publisher_client.publish("foo", idx).await?;
//     sleep(Duration::from_secs(1)).await;
// }

// publisher_client.quit().await?;
// subscriber_client.quit().await?;
// Ok(())
