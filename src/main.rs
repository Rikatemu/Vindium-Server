pub mod packet_manager;
pub mod types;

use packet_manager::AcceptData;
use rand::{thread_rng, distributions::Alphanumeric, Rng};
use tokio::{
    io::{AsyncWriteExt, AsyncReadExt},
    net::TcpListener, sync::broadcast
};

use crate::packet_manager::Packet;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    let (tx, _rx) = broadcast::channel(10);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (mut reader, mut writer) = socket.split();
            let mut buf = [0; 1024];
            let entity_id = generate_entity_id().await;

            let accept_data: AcceptData = AcceptData {
                accepted: true,
                entity_id,
                err_message: "".to_string()
            };

            // Send accept message
            writer.write_all(serde_json::to_string(&accept_data).unwrap().as_bytes()).await.unwrap();
    
            loop {
                tokio::select! {
                    result = reader.read(buf.as_mut()) => {
                        let n = result.unwrap();
                        if n == 0 {
                            break;
                        }

                        let packet: Result<Packet, serde_json::Error> = serde_json::from_str(&String::from_utf8_lossy(&buf[..n]));

                        match packet {
                            Ok(packet) => {
                                tx.send((packet, addr)).unwrap();
                            },
                            Err(e) => {
                                println!("Error: {:?}", e);
                            }
                        }
                    }
                    result = rx.recv() => {
                        let (packet, other_addr) = result.unwrap();

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

async fn generate_entity_id() -> String {
    return thread_rng()
    .sample_iter(&Alphanumeric)
    .take(30)
    .map(char::from)
    .collect();
}