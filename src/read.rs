use tokio::sync::broadcast;

use crate::packets::{packet::Packet, packet_reader::read_transform, data_types::PacketDataType};

// Packet reading and packet type handling
pub async fn handle_read_packet(
    packet_bytes: &[u8], 
    tx: broadcast::Sender<(Packet, std::net::SocketAddr)>,
    addr: std::net::SocketAddr
) {
    // TODO: Refactor duplicate code for sending packets to channel
    // Parse bytes to JSON string and then to Packet struct
    let ps_result = String::from_utf8(packet_bytes.to_vec());

    let mut packet_string = match ps_result {
        Ok(s) => s,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };

    packet_string = packet_string.trim_end_matches(char::from(0)).to_string();

    let packet: Result<Packet, serde_json::Error> = serde_json::from_str(&packet_string);
    match packet {
        Ok(packet) => {
            match packet.ptype {
                PacketDataType::Transform => {
                    let data = read_transform(packet).await;

                    let new_packet = Packet {
                        sender: addr.to_string(),
                        ptype: PacketDataType::Transform,
                        data: serde_json::to_string(&data).unwrap(),
                        send_back: false,
                        owner_only: false
                    };

                    let res = tx.send((new_packet, addr));
                    match res {
                        Ok(_) => return,
                        Err(e) => {
                            println!("Error: {:?}", e);
                            return;
                        }
                    }
                },
                PacketDataType::Ping => {
                    let new_packet = Packet {
                        sender: addr.to_string(),
                        ptype: PacketDataType::Ping,
                        data: "".to_string(),
                        send_back: true,
                        owner_only: true
                    };

                    let res = tx.send((new_packet, addr));
                    match res {
                        Ok(_) => return,
                        Err(e) => {
                            println!("Error: {:?}", e);
                            return;
                        }
                    }
                },
                _ => return
            }
        },
        Err(e) => {
            println!("Serde Error: {:?}", e);
            return;
        }
    }
}