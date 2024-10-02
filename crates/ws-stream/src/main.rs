use futures_util::StreamExt;
use std::io::Error;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex; // need to use this instead of std::sync::Mutex because we are in an async context
use types::WsMessage;

pub mod types;
pub mod user;
pub mod ws_manager;

use user::User;
use ws_manager::WsManager;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let addr = "127.0.0.1:4000".to_string();

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream) {
    let user_addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");

    println!("Address {}", user_addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    let (write, mut read) = ws_stream.split();

    // add a user whenever someone connects
    let user = User::new(user_addr.to_string(), write);
    let ws_manager = Arc::new(Mutex::new(WsManager::new().await));
    let mut manager = ws_manager.lock().await;
    manager.add_user(user);

    while let Some(msg) = read.next().await {
        let msg = msg.unwrap();
        if msg.is_text() {
            println!("Received a message: {}", msg);

            let msg = msg.to_text().unwrap();

            if let Ok(data) = serde_json::from_str::<WsMessage>(&msg) {
                process_data(
                    data,
                    user_addr.clone().to_string().as_str(),
                    ws_manager.clone(),
                )
                .await;
            }
        } else if msg.is_close() {
            println!("Closing Connection to user with addr: {}", user_addr);
            // remove user when connection is closed
            let mut manager = ws_manager.lock().await;
            manager.remove_user(user_addr.to_string().as_str());
        }
    }
}

async fn process_data(data: WsMessage, user_addr: &str, ws_manager: Arc<Mutex<WsManager>>) {
    let mut manager = ws_manager.lock().await;

    match data.method.as_str() {
        "SUBSCRIBE" => {
            manager.subscribe(user_addr, data).await;
        }
        "UNSUBSCRIBE" => {
            manager.unsubscribe(user_addr, data).await;
        }
        _ => {}
    }
}
