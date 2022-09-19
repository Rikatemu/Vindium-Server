pub mod types;
mod packet_manager;

use packet_manager::NetworkTransformData;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::{MutexGuard, Mutex};
use types::Entity;
use std::collections::HashMap;
use std::sync::Arc;

use crate::packet_manager::{Packet, AcceptData};

type EntityDb = Arc<Mutex<HashMap<String, Entity>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the server
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let mut connections: Arc<Mutex<Vec<Arc<Mutex<&TcpStream>>>>> = Arc::new(Mutex::new(Vec::new()));
    let entities = Arc::new(Mutex::new(types::Entities::new()));

    // Server loop
    loop {
        // Check for new connections and create a new thread for each one
        let (mut socket, _) = listener.accept().await?;
        let mut conns = connections.lock().await;
        for connection in conns.iter_mut() {
            if socket.peer_addr().ok() == connection.lock().await.peer_addr().ok() {
                let accept_message = AcceptData {
                    accepted: false,
                    entity_id: "".to_string(),
                    err_message: "Already connected".to_string()
                };
        
                let message = serde_json::to_string(&accept_message).unwrap();
                socket.write(message.as_bytes()).await.ok();

                println!("Connection already exists");
                continue;
            }
        }
        let stream = Arc::new(Mutex::new(&socket));
        conns.push(stream);

        let new_entity_id = generate_entity_id().await;

        println!("Connecting {}", socket.peer_addr().unwrap().to_string());

        let accept_message = AcceptData {
            accepted: true,
            entity_id: new_entity_id.clone(),
            err_message: String::from("")
        };

        let message = serde_json::to_string(&accept_message).unwrap();
        let res = socket.write(message.as_bytes()).await;
        let entities = entities.clone();
        let connections = connections.clone();

        match res {
            Ok(_) => {
                connect_client(socket, new_entity_id, entities, connections.clone()).await;
            },
            Err(e) => println!("Failed to send message to {} with error: {}", socket.peer_addr()?, e),
        }
    }
}

async fn connect_client(mut socket: TcpStream, new_entity_id: String, entities: EntityDb, connections: Arc<tokio::sync::Mutex<Vec<Arc<tokio::sync::Mutex<&tokio::net::TcpStream>>>>>) {
    // Create player entity
    let player_entity: Entity = Entity {
        id: new_entity_id.clone(),
        entity_type: types::EntityType::Player,
        owner: socket.peer_addr().unwrap().to_string(),
        pos: types::TransformPosition { x: 0.0, y: 0.0, z: 0.0 },
        rot: types::TransformRotation { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
    };

    println!("Creating entity id={} for {}", player_entity.id, player_entity.owner);

    let entity_db = &mut entities.lock().await;
    entity_db.insert(new_entity_id, player_entity);

    let entity_db = Arc::clone(&entities);
    // Handle incoming and outgoing packets for this client
    tokio::spawn(async move {
        let mut buf = [0; 1024];

        // In a loop, read data from the socket and write the data back.
        loop {
            let data = socket.read(buf.as_mut()).await;
            let n = match data {
                Ok(n) => if n.to_owned() == 0 { return; } else { n },
                Err(e) => {
                    println!("failed to read from socket; err = {:?}", e);
                    return;
                }
            };

            let packet: Result<Packet, serde_json::Error> = serde_json::from_slice(&buf[..n]);
            let entities = entity_db.lock().await;

            match packet {
                Ok(p) => handle_packet(p, entities, connections.to_owned()).await,
                Err(err) => println!("Failed to parse packet: {}", err),
            }
        }
    });
}

async fn handle_packet(packet: Packet, entities: MutexGuard<'_, HashMap<String, Entity>>, connections: Arc<Mutex<Vec<Arc<Mutex<&TcpStream>>>>>) {
    match packet.ptype {
        packet_manager::PacketType::Transform => handle_transform(packet, entities, connections).await,
        packet_manager::PacketType::Entity => todo!(),
    }
}

async fn handle_transform(packet: Packet, mut entities: MutexGuard<'_, HashMap<String, Entity>>, connections: Arc<Mutex<Vec<Arc<Mutex<&TcpStream>>>>>) {
    let res: Result<NetworkTransformData, serde_json::Error> = serde_json::from_str(&packet.data.as_str());
    let data: &NetworkTransformData = match &res {
        Ok(d) => d,
        Err(e) => {
            println!("Failed to parse transform packet: {}", e);
            return;
        }
    };

    for entity in entities.values_mut() {
        if entity.id == data.entity_id {
            entity.pos = data.position.clone();
            entity.rot = data.rotation.clone();

            sync_transform(entity.owner.to_owned(), packet, data, connections).await;
            return;
        }
    }

    println!("Failed to find entity with id {}", data.entity_id);
}

async fn sync_transform(owner: String, owner_packet: Packet, data: &NetworkTransformData, connections: Arc<Mutex<Vec<Arc<Mutex<&TcpStream>>>>>) {
    for connection in connections.lock().await.iter_mut() {
        // Sync only to other clients, not the owner
        if owner != connection.lock().await.peer_addr().unwrap().to_string() {
            let packet = Packet {
                ptype: packet_manager::PacketType::Transform,
                data: serde_json::to_string(data).unwrap(),
                sender: owner_packet.sender.clone(),
            };
    
            let message = serde_json::to_string(&packet).unwrap();
            connection.lock().await.write(message.as_bytes()).await.ok();
        }
    }
}

async fn generate_entity_id() -> String {
    return thread_rng()
    .sample_iter(&Alphanumeric)
    .take(30)
    .map(char::from)
    .collect();
}