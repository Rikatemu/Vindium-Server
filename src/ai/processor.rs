use std::{collections::HashMap, sync::Arc, net::SocketAddr};

use tokio::{time::sleep, sync::Mutex};

use crate::{config::MIN_TICK_LENGTH_MS, networking::{types::{Entity, EntityType}, packets::packet::Packet}, ai::behaviours::passive_ground};

pub async fn run(
    entities: Arc<Mutex<HashMap<EntityType, HashMap<String, Entity>>>>,
    processor_id: u8,
    tx: tokio::sync::broadcast::Sender<(Packet, SocketAddr)>,
) {
    loop {
        let time_tick_start = tokio::time::Instant::now();

        let ents = Arc::clone(&entities);
        let tx = tx.clone();

        for (entity_type, entity_map) in ents.lock().await.iter_mut() {
            let tx = tx.clone();
            if entity_type == &EntityType::Ai {
                for (entity_key, entity) in entity_map.iter_mut() {
                    let tx = tx.clone();
                    let ai_data = entity.ai_data.as_mut();
                    match ai_data {
                        Some(ai_data) => {
                            if ai_data.processor_id == processor_id {
                                match ai_data.behaviour {
                                    super::behaviours::AiBehaviourType::PassiveGround => passive_ground::update(entity, tx).await,
                                };
                            }
                        },
                        None => {
                            println!("Entity {} has no AI data", entity_key);
                        }
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
}