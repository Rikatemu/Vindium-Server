use std::{collections::HashMap, sync::Arc, net::SocketAddr};

use tokio::sync::Mutex;

use crate::networking::{types::{EntityType, Entity}, packets::packet::Packet};

use super::processor;

pub const NUMBER_OF_PROCESSORS: u8 = 1;
pub const AI_ENTITIES_PER_PROCESSOR: u8 = 100;

// Initialize AI processors on separate threads
pub async fn initialize_ai_processors(
    entities: Arc<Mutex<HashMap<EntityType, HashMap<String, Entity>>>>,
    tx: tokio::sync::broadcast::Sender<(Packet, SocketAddr)>,
) {
    for processor_id in 0..=NUMBER_OF_PROCESSORS {
        let ents = Arc::clone(&entities);
        let tx = tx.clone();
        tokio::spawn(async move {
            processor::run(ents, processor_id, tx).await;
        });
    }
}