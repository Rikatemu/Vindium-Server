use std::time::Duration;

use crate::networking::types::{Vector3, Quaternion};

pub const SERVER_PORT: &str = "8080";

pub const SPAWN_POINT: Vector3 = Vector3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

pub const SPAWN_POINT_ROT: Quaternion = Quaternion {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    w: 0.0,
};

// 30 ticks per 1 second - 33.3333ms per tick (1000 / 30)
pub const TICK_RATE: u64 = 30;
pub const MIN_TICK_LENGTH_MS: Duration = Duration::from_millis(1000 / TICK_RATE);

pub const BUFFER_SIZE: usize = 2048;