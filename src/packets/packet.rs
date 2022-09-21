use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Packet {
    pub sender: String,
    pub ptype: PacketType,
    pub data: String,
    pub send_back: bool,
    pub owner_only: bool
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum PacketType {
    Accept,
    Transform,
    Spawn,
    Disconnect,
    Ping,
    Chat
}