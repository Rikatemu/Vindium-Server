use serde::{Serialize, Deserialize};

use crate::types::{TransformPosition, TransformRotation};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum PacketType {
    Transform,
    Entity,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    pub sender: String,
    pub ptype: PacketType,
    pub data: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkTransformData {
    pub entity_id: String,
    pub position: TransformPosition,
    pub rotation: TransformRotation,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AcceptData {
    pub accepted: bool,
    pub entity_id: String,
    pub err_message: String
}