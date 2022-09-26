use crate::networking::packets::{
    data_types::{DisconnectData, SpawnData, AcceptData}, 
    packet::Packet
};
use crate::config::MIN_TICK_LENGTH_MS;
use crate::networking::read::handle_read_packet;
use crate::config::{SPAWN_POINT, SPAWN_POINT_ROT};
use crate::networking::helper::generate_entity_id;
use std::{net::SocketAddr, collections::HashMap, sync::Arc};

use tokio::{net::TcpStream, sync::{broadcast::{Sender, Receiver}, Mutex}, io::{AsyncWriteExt, AsyncReadExt}, time::sleep};

use super::{packets::data_types::PacketDataType, types::{Entity, EntityType}};

pub async fn handle_client(
    mut socket: TcpStream, 
    addr: SocketAddr, 
    tx: Sender<(Packet, SocketAddr)>, 
    mut rx: Receiver<(Packet, SocketAddr)>,
    entities: Arc<Mutex<HashMap<EntityType, HashMap<String, Entity>>>>
) {
    // Spawn a new task to handle the client connection
    tokio::spawn(async move {
        // Split socket to reader and writer
        let (mut reader, mut writer) = socket.split();

        // Create player entity
        let entity_id = generate_entity_id().await;

        let player_entity = Entity {
            id: entity_id.clone(),
            entity_type: EntityType::Player,
            owner: addr.to_string(),
            pos: SPAWN_POINT,
            rot: SPAWN_POINT_ROT,
            ai_data: None
        };

        // Add player entity to player entities hashmap
        let mut entities_lock = entities.lock().await;
        let player_entities = entities_lock.entry(EntityType::Player).or_insert(HashMap::new());
        player_entities.insert(entity_id.clone(), player_entity);
        // Drop lock otherwise it will be held until the end of the function which happens on client disconnection
        drop(entities_lock);


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
            let time_tick_start = tokio::time::Instant::now();

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

                                // Remove player entity from player entities hashmap
                                let mut entities_lock = entities.lock().await;
                                let player_entities = entities_lock.entry(EntityType::Player).or_insert(HashMap::new());
                                player_entities.remove(&entity_id);
                                drop(entities_lock);

                                // Send disconnect packet to other clients
                                let packet = Packet {
                                    sender: addr.to_string(),
                                    ptype: PacketDataType::Disconnect,
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

            // Sleep for the remaining time of the tick
            let time_elapsed = time_tick_start.elapsed();
            if time_elapsed < MIN_TICK_LENGTH_MS {
                sleep(MIN_TICK_LENGTH_MS - time_elapsed).await;
            }
        }
    });
}