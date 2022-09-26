pub mod config;
pub mod networking;
pub mod ai;

use crate::networking::types::Entity;
use std::{collections::HashMap, sync::Arc, net::SocketAddr};

use config::SERVER_PORT;
use networking::{types::EntityType, packets::packet::Packet};
use tokio::{net::TcpListener, sync::{broadcast::{self, Sender}, Mutex}};

use crate::{
    networking::client::handle_client, 
    ai::processor_controller::initialize_ai_processors
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:".to_owned() + SERVER_PORT).await.unwrap_or_else(|e| {
        println!("Error: {:?}", e);
        panic!("Failed to bind to address");
    });
    println!("Server started on port {}", SERVER_PORT);

    // Store entities in a hashmap
    let entities: Arc<Mutex<HashMap<EntityType, HashMap<String, Entity>>>> = Arc::new(Mutex::new(HashMap::new()));

    // Create a channel to send messages to all clients
    let (tx, _rx) = broadcast::channel(10000);
    let tx: Sender<(Packet, SocketAddr)> = tx;

    // Initialize AI processors
    let tx_ai = tx.clone();
    let ents = Arc::clone(&entities);
    initialize_ai_processors(ents, tx_ai).await;
    println!("AI processors initialized");

    // Add a test entity ------------------------------------------------------------------------------
    let entity1 = Entity {
        id: "test1".to_string(),
        entity_type: EntityType::Ai,
        owner: "server".to_string(),
        pos: crate::networking::types::Vector3 { x: 0.0, y: 0.0, z: 0.0 },
        rot: crate::networking::types::Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
        ai_data: Some(crate::networking::types::AiEntityData {
            processor_id: 0,
            behaviour: crate::ai::behaviours::AiBehaviourType::PassiveGround,
        }),
    };

    let entity2 = Entity {
        id: "test2".to_string(),
        entity_type: EntityType::Ai,
        owner: "server".to_string(),
        pos: crate::networking::types::Vector3 { x: 5.0, y: 0.0, z: 5.0 },
        rot: crate::networking::types::Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
        ai_data: Some(crate::networking::types::AiEntityData {
            processor_id: 0,
            behaviour: crate::ai::behaviours::AiBehaviourType::PassiveGround,
        }),
    };

    let ents = Arc::clone(&entities);
    let mut ents_lock = ents.lock().await;
    let ai_entities = ents_lock.entry(EntityType::Ai).or_insert(HashMap::new());
    ai_entities.insert(entity1.id.clone(), entity1);
    ai_entities.insert(entity2.id.clone(), entity2);
    println!("Test entity added");
    drop(ents_lock);
    // End test entity --------------------------------------------------------------------------------

    // Loop to acquire new client connections
    let tx = tx.clone();
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

        let ents = Arc::clone(&entities);

        // Spawn a new task to handle the client connection
        handle_client(socket, addr, tx, rx, ents).await;
    }
}