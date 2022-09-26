use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::ai::behaviours::AiBehaviourType;

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
#[derive(Eq, PartialEq)]
pub enum EntityType {
    Player,
    Ai,
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    pub id: String,
    pub entity_type: EntityType,
    pub owner: String,
    pub pos: Vector3,
    pub rot: Quaternion,
    pub ai_data: Option<AiEntityData>,
}

impl Entity {
    pub fn clone(self) -> Entity {
        Entity {
            id: self.id,
            entity_type: self.entity_type,
            owner: self.owner,
            pos: self.pos,
            rot: self.rot,
            ai_data: self.ai_data,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AiEntityData {
    pub processor_id: u8,
    pub behaviour: AiBehaviourType,
}

pub type Entities = HashMap<String,Entity>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}