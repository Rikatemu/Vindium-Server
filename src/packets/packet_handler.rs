use crate::types::{TransformPosition, TransformRotation};

use super::packet_manager::{NetworkTransformData, Packet};

pub async fn handle_transform(packet: Packet) -> NetworkTransformData {
    let data: NetworkTransformData = serde_json::from_str(&packet.data).unwrap_or_else(|e| {
        println!("Error: {:?}", e);
        NetworkTransformData {
            entity_id: "0".to_string(),
            position: TransformPosition {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: TransformRotation {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
        }
    });
    
    return data;
}