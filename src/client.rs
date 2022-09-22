use std::net::SocketAddr;

use tokio::{net::TcpStream, sync::broadcast::{Sender, Receiver}, io::{AsyncWriteExt, AsyncReadExt}};

use crate::{packets::{packet::Packet, data_types::{AcceptData, SpawnData, DisconnectData, PacketDataType}}, helper::generate_entity_id, read::handle_read_packet, config::{SPAWN_POINT, SPAWN_POINT_ROT}};

pub async fn handle_client(mut socket: TcpStream, addr: SocketAddr, tx: Sender<(Packet, SocketAddr)>, mut rx: Receiver<(Packet, SocketAddr)>) {
    // Spawn a new task to handle the client connection
    tokio::spawn(async move {
        // Split socket to reader and writer
        let (mut reader, mut writer) = socket.split();

        // Generate a random ID for the client entity
        let entity_id = generate_entity_id().await;

        // Send the client their spawn data and accept the connection
        let accept_data: AcceptData = AcceptData {
            accepted: true,
            entity_id: entity_id.clone(),
            err_message: "".to_string(),
            spawn_data: SpawnData {
                entity_id: entity_id.clone(),
                position: SPAWN_POINT,
                rotation: SPAWN_POINT_ROT
            },
        };

        // Send accept message
        writer.write(serde_json::to_string(&accept_data).unwrap().as_bytes()).await.unwrap();

        // Send spawn message to all other clients
        let spawn_data: SpawnData = SpawnData {
            entity_id: entity_id.clone(),
            position: SPAWN_POINT,
            rotation: SPAWN_POINT_ROT
        };

        let spawn_packet = Packet {
            sender: addr.to_string(),
            ptype: PacketDataType::Spawn,
            data: serde_json::to_string(&spawn_data).unwrap(),
            send_back: false,
            owner_only: false
        };

        let res = tx.send((spawn_packet, addr));

        match res {
            Ok(_) => {},
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }

        // Loop for handling of incoming and outgoing messages
        loop {
            let mut buf: [u8; 4096]  = [0; 4096];
            let tx = tx.clone();
            tokio::select! {
                // Handle incoming messages from the client
                result = reader.read(&mut buf) => {
                    match result {
                        Ok(n) => {
                            if n == 0 {
                                println!("Client disconnected: {}", addr);

                                let disconnect_data = DisconnectData {
                                    entity_id: entity_id.clone()
                                };

                                // Send disconnect packet to other clients
                                let packet = Packet {
                                    sender: addr.to_string(),
                                    ptype: crate::packets::data_types::PacketDataType::Disconnect,
                                    data: serde_json::to_string(&disconnect_data).unwrap(),
                                    send_back: false,
                                    owner_only: false
                                };

                                let res = tx.send((packet, addr));

                                match res {
                                    Ok(_) => break,
                                    Err(e) => {
                                        println!("Error: {:?}", e);
                                        break;
                                    }
                                }
                            }

                            handle_read_packet(&mut buf[..n], tx, addr).await;
                        },
                        Err(e) => {
                            println!("Error: {:?}", e);
                            continue;
                        }
                    }
                }

                // Handle outgoing messages to the client
                result = rx.recv() => {
                    match result {
                        Ok(msg) => {
                            let (packet, other_addr) = msg;

                            /*
                            * TODO: Refactor this ugly thing
                            * If the packet is not meant to be sent back to the sender,
                            * don't send it back to the sender address
                            */
                            if packet.send_back {
                                if packet.owner_only {
                                    if packet.sender == addr.to_string() {
                                        let res = writer.write(serde_json::to_string(&packet).unwrap().as_bytes()).await;
                                        if res.is_err() {
                                            println!("Error: {:?}", res.err().unwrap());
                                            continue;
                                        }
                                    }
                                } else {
                                    let res = writer.write(serde_json::to_string(&packet).unwrap().as_bytes()).await;
                                    if res.is_err() {
                                        println!("Error: {:?}", res.err().unwrap());
                                        continue;
                                    }
                                }
                            } else if other_addr != addr {
                                let res = writer.write(serde_json::to_string(&packet).unwrap().as_bytes()).await;
                                if res.is_err() {
                                    println!("Error: {:?}", res.err().unwrap());
                                    continue;
                                }
                            }
                        }
                        Err(e) => {
                            println!("Error: {:?}", e);
                            continue;
                        }
                    }
                }
            }
        }
    });
}