use serde::{Serialize, Deserialize};

use crate::types::{Vector3, Quaternion};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum PacketDataType {
    Accept,
    Transform,
    Spawn,
    Disconnect,
    Ping,
    Chat
}

// Packet data types ----------------------------------------------
#[derive(Serialize, Deserialize, Debug)]
pub struct AcceptData {
    pub accepted: bool,
    pub entity_id: String,
    pub err_message: String,
    pub spawn_data: SpawnData
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransformData {
    pub entity_id: String,
    pub position: Vector3,
    pub rotation: Quaternion,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpawnData {
    pub entity_id: String,
    pub position: Vector3,
    pub rotation: Quaternion,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DisconnectData {
    pub entity_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PingData {
    pub entity_id: String,
    pub ping_code: String,
}