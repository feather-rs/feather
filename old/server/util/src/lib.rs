#![forbid(unsafe_code)]

//! Assorted utility functions and trivial game logic.

use arrayvec::ArrayVec;
use feather_core::util::{BlockPosition, ChunkPosition, Position};
use nalgebra_glm::{vec3, DVec3};

mod block;
pub use block::*;
mod chunk_entities;
pub use chunk_entities::*;
mod time;
pub use time::*;
mod load;
pub use load::*;

use feather_server_types::{Game, Uuid};
use fecs::{Entity, World};
use rand::Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::time::{SystemTime, UNIX_EPOCH};

/// Retrieves the current time in seconds
/// since the UNIX epoch.
pub fn current_time_in_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Retrieves the current time in milliseconds
/// since the UNIX epoch.
pub fn current_time_in_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

/// Calculates the relative move fields
/// as used in the Entity Relative Move packets.
pub fn calculate_relative_move(old: Position, current: Position) -> (i16, i16, i16) {
    let x = ((current.x * 32.0 - old.x * 32.0) * 128.0) as i16;
    let y = ((current.y * 32.0 - old.y * 32.0) * 128.0) as i16;
    let z = ((current.z * 32.0 - old.z * 32.0) * 128.0) as i16;
    (x, y, z)
}

/// Converts degrees to stops as used in the protocol.
pub fn degrees_to_stops(degs: f32) -> u8 {
    ((degs / 360.0) * 256.0) as u8
}

/// Returns the set of block positions adjacent to a given position.
pub fn adjacent_blocks(pos: BlockPosition) -> ArrayVec<[BlockPosition; 6]> {
    [
        pos + BlockPosition::new(1, 0, 0),
        pos + BlockPosition::new(0, 1, 0),
        pos + BlockPosition::new(0, 0, 1),
        pos + BlockPosition::new(-1, 0, 0),
        pos + BlockPosition::new(0, -1, 0),
        pos + BlockPosition::new(0, 0, -1),
    ]
    .iter()
    .filter(|pos| pos.y >= 0 && pos.y < 256)
    .copied()
    .collect()
}

/// Converts float-based velocity in blocks per tick
/// to the format used by the protocol.
pub fn protocol_velocity(vel: DVec3) -> (i16, i16, i16) {
    // These are in units of 1/8000 block per tick.
    (
        (vel.x * 8000.0) as i16,
        (vel.y * 8000.0) as i16,
        (vel.z * 8000.0) as i16,
    )
}

/// Returns all entities within the given distance of the given
/// position.
///
/// # Panics
/// Panics if either coordinate of the radius is negative.
pub fn nearby_entities(
    world: &World,
    game: &Game,
    pos: Position,
    radius: DVec3,
) -> SmallVec<[Entity; 4]> {
    assert!(radius.x >= 0.0);
    assert!(radius.y >= 0.0);
    assert!(radius.z >= 0.0);

    let mut result = SmallVec::new();

    for chunk in chunks_within_distance(pos, radius) {
        let entities = game.chunk_entities.entities_in_chunk(chunk);
        entities
            .iter()
            .copied()
            .filter(|e| {
                let epos = world.try_get::<Position>(*e);
                if let Some(epos) = epos {
                    (epos.x - pos.x).abs() <= radius.x
                        && (epos.y - pos.y).abs() <= radius.y
                        && (epos.z - pos.z).abs() <= radius.z
                } else {
                    false
                }
            })
            .for_each(|e| result.push(e));
    }

    result
}

/// Finds all chunks within a given distance (in blocks)
/// of a position.
///
/// The Y coordinate of `distance` is ignored.
pub fn chunks_within_distance(
    mut pos: Position,
    mut distance: DVec3,
) -> SmallVec<[ChunkPosition; 9]> {
    assert!(distance.x >= 0.0);
    assert!(distance.z >= 0.0);

    let mut result = SmallVec::new();

    let mut x_len = 0;
    let mut z_len = 0;

    let center_chunk_pos = pos.chunk();

    loop {
        let needed = ((pos.x + 16.0) / 16.0).floor() * 16.0 - pos.x;
        if needed > distance.x {
            break;
        }

        distance.x -= needed;
        pos.x += needed;
        x_len += 1;
    }

    loop {
        let needed = ((pos.z + 16.0) / 16.0).floor() * 16.0 - pos.z;
        if needed > distance.z {
            break;
        }

        distance.z -= needed;
        pos.z += needed;
        z_len += 1;
    }

    for x in -x_len..=x_len {
        for z in -z_len..=z_len {
            result.push(ChunkPosition::new(
                x + center_chunk_pos.x,
                z + center_chunk_pos.z,
            ));
        }
    }

    result
}

pub fn charge_from_ticks_held(ticks: u32) -> f32 {
    let ticks = ticks as f32;

    let mut unbounded_force = (ticks * (ticks + 40.0)) / 400.0;

    if unbounded_force > 3.0 {
        unbounded_force = 3.0
    }

    unbounded_force
}

pub fn compute_projectile_velocity(
    direction: DVec3,
    charge: f64,
    inaccuracy: f64,
    rng: &mut impl Rng,
) -> DVec3 {
    let gaussian = vec3(
        StandardNormal.sample(rng),
        StandardNormal.sample(rng),
        StandardNormal.sample(rng),
    );
    let inaccuracy = vec3(inaccuracy, inaccuracy, inaccuracy).component_mul(&gaussian) * 0.0075;

    (direction + inaccuracy) * charge
}

/// Compute offline mode UUID
/// https://gist.github.com/games647/2b6a00a8fc21fd3b88375f03c9e2e603
pub fn name_to_uuid_offline(username: &str) -> Uuid {
    let mut context = md5::Context::new();
    context.consume(format!("OfflinePlayer:{}", username).as_bytes());
    let computed = context.compute();
    let bytes = computed.into();

    let mut builder = uuid::Builder::from_bytes(bytes);

    builder
        .set_variant(uuid::Variant::RFC4122)
        .set_version(uuid::Version::Md5);

    builder.build()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProfileResponse {
    id: Uuid,
    name: String,
}

/// Gets the UUID of a username by requesting it from Mojang's API
pub async fn name_to_uuid_online(username: &str) -> Option<Uuid> {
    let auth_result = reqwest::get(&format!(
        "https://api.mojang.com/users/profiles/minecraft/{}",
        username
    ))
    .await;

    match auth_result {
        Ok(res) => match res.json::<ProfileResponse>().await {
            Ok(json) => Some(json.id),
            Err(_) => None,
        },
        Err(_) => None,
    }
}
