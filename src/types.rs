use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EntityType {
    Player,
    Npc,
    Item,
    Projectile,
    Vehicle
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    pub id: String,
    pub entity_type: EntityType,
    pub owner: String,
    pub pos: Vector3,
    pub rot: Quaternion,
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