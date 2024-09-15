use fred::{clients::SubscriberClient, prelude::*};

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

    pub async fn pop(&self, key: &str, count: Option<usize>) -> Result<Vec<RedisValue>, RedisError> {
        self.client.rpop(key, count).await
    }

    pub async fn publish(&self, channel: &str, value: String) -> Result<(), RedisError> {
        self.publisher.publish(channel, value).await
    }

    pub async fn subscribe(&self, channel: &str) -> Result<(), RedisError> {
        self.subscriber.subscribe(channel).await
    }
}

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
