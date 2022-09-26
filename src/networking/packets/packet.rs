use serde::{Serialize, Deserialize};

use super::data_types::PacketDataType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Packet {
    pub sender: String,
    pub ptype: PacketDataType,
    pub data: String,
    pub send_back: bool,
    pub owner_only: bool
}