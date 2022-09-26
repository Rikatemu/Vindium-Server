use serde::{Serialize, Deserialize};

pub mod passive_ground;

#[derive(Serialize, Deserialize, Debug)]
pub enum AiBehaviourType {
    PassiveGround,
}