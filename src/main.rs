pub mod types;
pub mod packets;
pub mod read;
pub mod helper;
pub mod client;

use tokio::{net::TcpListener, sync::broadcast};

use crate::client::handle_client;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap_or_else(|e| {
        println!("Error: {:?}", e);
        panic!("Failed to bind to address");
    });

    // Create a channel to send messages to all clients
    let (tx, _rx) = broadcast::channel(10);

    // Loop to acquire new client connections
    loop {
        // Accept a new client connection
        let client = listener.accept().await;

        // Create a new channel to send messages to the client
        let (socket, addr) = match client {
            Ok((socket, addr)) => (socket, addr),
            Err(e) => {
                println!("Error: {:?}", e);
                continue;
            }
        };

        println!("New client connected: {}", addr);

        let tx = tx.clone();
        let rx = tx.subscribe();

        // Spawn a new task to handle the client connection
        handle_client(socket, addr, tx, rx).await;
    }
}