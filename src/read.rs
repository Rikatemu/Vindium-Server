use tokio::sync::broadcast;

use crate::packets::{packet::{Packet, PacketType}, packet_handler::read_transform};

// Packet reading and packet type handling
pub async fn handle_read_packet(
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
                    let data = read_transform(packet).await;

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