use crate::types::{Vector3, Quaternion};

use super::{data_types::TransformData, packet::Packet};

pub async fn read_transform(packet: Packet) -> TransformData {
    let data: TransformData = serde_json::from_str(&packet.data).unwrap_or_else(|e| {
        println!("Error: {:?}", e);
        TransformData {
            entity_id: "0".to_string(),
            position: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: Quaternion {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
        }
    });
    
    return data;
}