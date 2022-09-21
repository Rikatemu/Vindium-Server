use serde::{Serialize, Deserialize};

use crate::types::{TransformPosition, TransformRotation};

// Packet data types ----------------------------------------------
#[derive(Serialize, Deserialize, Debug)]
pub struct AcceptData {
    pub accepted: bool,
    pub entity_id: String,
    pub err_message: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransformData {
    pub entity_id: String,
    pub position: TransformPosition,
    pub rotation: TransformRotation,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpawnData {
    pub entity_id: String,
    pub position: TransformPosition,
    pub rotation: TransformRotation,
}