//! Implements particle entities.

use feather_core::misc::ParticleData;
use feather_core::network::{packets, Packet};
use feather_core::util::Position;
use feather_server_types::{ParticleCount, SpawnPacketCreator};
use fecs::{EntityBuilder, EntityRef};

/// Creates a particle with the given kind and count.
pub fn create(kind: ParticleData, count: u32) -> EntityBuilder {
    crate::base()
        .with(kind)
        .with(ParticleCount(count))
        .with(SpawnPacketCreator(&create_spawn_packet))
}

fn create_spawn_packet(accessor: &EntityRef) -> Box<dyn Packet> {
    let particle = accessor.get::<ParticleData>();
    let count = accessor.get::<ParticleCount>().0;
    let pos = *accessor.get::<Position>();

    Box::new(packets::Particle {
        long_distance: false,
        x: pos.x as f32,
        y: pos.y as f32,
        z: pos.z as f32,
        offset_x: 0.0, // TODO: offsets
        offset_y: 0.0,
        offset_z: 0.0,
        particle_data: 0.0, // TODO: what is this?
        particle_count: count as i32,
        data: *particle,
    })
}
