use actix_web::error;

pub async fn enqueue_message() -> actix_web::Result<String> {
    let redis_client = redis::Client::open("redis://127.0.0.1:6380/").unwrap();

    let mut conn = redis_client
        .get_connection_manager()
        .await
        .map_err(error::ErrorInternalServerError)?;

    let res: isize = redis::cmd("LPUSH")
        .arg("my_queue")
        .arg("hello")
        .query_async(&mut conn)
        .await
        .map_err(error::ErrorInternalServerError)?;

    if res > 0 {
        Ok("Message enqueued successfully".to_string())
    } else {
        Err(error::ErrorInternalServerError("Failed to enqueue message"))
    }
}

pub async fn dequeue_message() -> actix_web::Result<String> {
    let redis_client = redis::Client::open("redis://127.0.0.1:6380/").unwrap();

    let mut conn = redis_client
        .get_connection_manager()
        .await
        .map_err(error::ErrorInternalServerError)?;

    let res: Option<(String, String)> = redis::cmd("BRPOP")
        .arg("my_queue")
        .arg(0) // Block indefinitely
        .query_async(&mut conn)
        .await
        .map_err(error::ErrorInternalServerError)?;

    match res {
        Some((_, message)) => Ok(message),
        None => Err(error::ErrorInternalServerError("Failed to dequeue message"))
    }
}

pub async fn publish_message() -> actix_web::Result<String> {
    let redis_client = redis::Client::open("redis://127.0.0.1:6380/").unwrap();
    
    let mut conn = redis_client
        .get_connection_manager()
        .await
        .map_err(error::ErrorInternalServerError)?;

    let _: () = redis::cmd("PUBLISH")
        .arg("channel1") // channel name
        .arg("message1") // message
        .query_async(&mut conn)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok("Message published successfully".to_string())
}

pub async fn subscribe_to_channel() -> actix_web::Result<String> {
    let redis_client = redis::Client::open("redis://127.0.0.1:6380/").unwrap();
    
    let mut conn = redis_client
        .get_async_connection()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let mut pubsub_conn = conn.into_pubsub();
    pubsub_conn
        .subscribe("channel1")
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    
    let messages = pubsub_conn
        .into_on_message() // This creates a stream of messages
        .map(|msg| msg.get_payload::<String>())
        .filter_map(|result| async move { result.ok() })
        .map(|result| result.unwrap());

    if let Some(message) = messages.next().await {
        let payload: String = message.get_payload().map_err(actix_web::error::ErrorInternalServerError)?;
        Ok(format!("Received message: {}", payload))
    } else {
        Err(actix_web::error::ErrorInternalServerError("Failed to receive message"))
    }
}