use std::net::SocketAddr;

use crate::networking::{types::Entity, packets::{packet::Packet, data_types::{PacketDataType, TransformData}}};

// Update runs once per tick for each entity
pub async fn update(
    entity: &mut Entity, 
    tx: tokio::sync::broadcast::Sender<(Packet, SocketAddr)>
) {
    entity.pos.x += 0.05;

    sync_to_clients(entity, tx).await;
}

pub async fn sync_to_clients(
    entity: &mut Entity, 
    tx: tokio::sync::broadcast::Sender<(Packet, SocketAddr)>
) {
    // Prepare transform data
    let data = TransformData {
        entity_id: entity.id.clone(),
        position: entity.pos.clone(),
        rotation: entity.rot.clone(),
    };

    // Prepare packet
    let new_packet = Packet {
        sender: "0.0.0.0:8080".to_string(),
        ptype: PacketDataType::Transform,
        data: serde_json::to_string(&data).unwrap(),
        send_back: false,
        owner_only: false
    };

    // Send packet
    tx.send((new_packet, "127.0.0.1:8080".parse().unwrap())).unwrap();
}