use std::net::SocketAddr;

use tokio::{net::TcpStream, sync::broadcast::{Sender, Receiver}, io::{AsyncWriteExt, AsyncReadExt}};

use crate::{packets::{packet::Packet, data_types::AcceptData}, helper::generate_entity_id, read::handle_read_packet};

type Buffer = [u8; 1024];

pub async fn handle_client(mut socket: TcpStream, addr: SocketAddr, tx: Sender<(Packet, SocketAddr)>, mut rx: Receiver<(Packet, SocketAddr)>) {
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
                    match result {
                        Ok(n) => {
                            if n == 0 {
                                break;
                            }

                            handle_read_packet(&buf[..n], tx, addr).await;
                        },
                        Err(e) => {
                            println!("Error: {:?}", e);
                            break;
                        }
                    }
                }

                // Handle outgoing messages to the client
                result = rx.recv() => {
                    match result {
                        Ok(msg) => {
                            let (packet, other_addr) = msg;

                            // Do NOT send the packet back to the client that sent it
                            if addr != other_addr {
                                let new_packet = serde_json::to_string(&packet).unwrap();
                                let res = writer.write_all(new_packet.as_bytes()).await;
                                if res.is_err() {
                                    println!("Error: {:?}", res.err().unwrap());
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            println!("Error: {:?}", e);
                            break;
                        }
                    }
                }
            }
        }
    });
}