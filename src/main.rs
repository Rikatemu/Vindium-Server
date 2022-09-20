pub mod types;
pub mod packets;

use crate::packets::packet_handler::handle_transform;
use crate::packets::packet_manager::{Packet, AcceptData};
use packets::packet_manager::PacketType;
use rand::{thread_rng, distributions::Alphanumeric, Rng};
use tokio::{
    io::{AsyncWriteExt, AsyncReadExt},
    net::TcpListener, sync::broadcast
};

type Buffer = [u8; 1024];

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
        let (mut socket, addr) = match client {
            Ok((socket, addr)) => (socket, addr),
            Err(e) => {
                println!("Error: {:?}", e);
                continue;
            }
        };

        println!("New client connected: {}", addr);

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        // Spawn a new task to handle the client connection
        tokio::spawn(async move {
            // Split socket to reader and writer
            let (mut reader, mut writer) = socket.split();

            // Create a buffer to store the data
            let mut buf: Buffer = [0; 1024];

            // Generate a random ID for the client entity
            let entity_id = generate_entity_id().await;

            // Send the client their entity ID and accept the connection
            let accept_data: AcceptData = AcceptData {
                accepted: true,
                entity_id,
                err_message: "".to_string()
            };

            // Send accept message
            writer.write_all(serde_json::to_string(&accept_data).unwrap().as_bytes()).await.unwrap();
    
            // Loop for handling of incoming and outgoing messages
            loop {
                let tx = tx.clone();
                tokio::select! {
                    // Handle incoming messages from the client
                    result = reader.read(buf.as_mut()) => {
                        let n = result.unwrap();
                        if n == 0 {
                            break;
                        }

                        handle_read_packet(&buf[..n], tx, addr).await;
                    }
                    // Handle outgoing messages to the client
                    result = rx.recv() => {
                        let (packet, other_addr) = result.unwrap();

                        // Do NOT send the packet back to the client that sent it
                        if addr != other_addr {
                            let new_packet = serde_json::to_string(&packet).unwrap();
                            writer.write_all(new_packet.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}

// Packet reading and packet type handling
async fn handle_read_packet(
    packet_bytes: &[u8], 
    tx: broadcast::Sender<(Packet, std::net::SocketAddr)>,
    addr: std::net::SocketAddr
) {
    // Parse bytes to JSON string and then to Packet struct
    let packet_string = String::from_utf8_lossy(&packet_bytes);
    let packet: Result<Packet, serde_json::Error> = serde_json::from_str(&packet_string);
    match packet {
        Ok(packet) => {
            match packet.ptype {
                PacketType::Transform => {
                    let data = handle_transform(packet).await;

                    let new_packet = Packet {
                        sender: data.entity_id.clone(),
                        ptype: PacketType::Transform,
                        data: serde_json::to_string(&data).unwrap()
                    };

                    tx.send((new_packet, addr)).unwrap();
                },
                _ => {}
            }
        },
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}


// Generate a random ID for the entity
async fn generate_entity_id() -> String {
    return thread_rng()
    .sample_iter(&Alphanumeric)
    .take(30)
    .map(char::from)
    .collect();
}