//! Assorted utilities for use in Feather's codebase.
use bumpalo::Bump;
use feather_core::{ChunkPosition, ItemStack, Packet, Position};
use glm::DVec3;
use spawn::Spawner;
use thread_local::ThreadLocal;

#[macro_use]
mod macros;
mod broadcaster;
mod spawn;

use broadcaster::Broadcaster;
pub use broadcaster::BroadcasterSystem;
pub use macros::*;
pub use spawn::SpawnerSystem;
use specs::Entity;

/// Converts float-based velocity in blocks per tick
/// to the obnoxious format used by the protocol.
pub fn protocol_velocity(vel: DVec3) -> (i16, i16, i16) {
    // Apparently, these are in units of 1/8000 block per tick.
    (
        (vel.x / 8000.0) as i16,
        (vel.y / 8000.0) as i16,
        (vel.z / 8000.0) as i16,
    )
}

/// General-purpose utility resource, used to
/// ergonomically perform various actions without
/// specifying ridiculous amounts of system dependencies.
///
/// This struct is thread-safe and should never be used
/// as a `Write` dependency.
///
/// # Available functions
/// * `alloc` - used to allocate temporary values
/// using a bump allocator. When allocating values
/// which will only be used inside a function, for example,
/// this function should be used rather than allocating
/// directly on the heap.
/// * `spawn_*` - functions to lazily spawn entities of
/// various types. This avoids the need to specify write
/// storage dependencies for each component the entity
/// needs. Note, however, that the entity isn't created
/// until the handling dispatcher stage. These functions simply
/// redirect to `entity::Spawner`.
/// * `broadcast` - lazily broadcasts a packet to all players
/// who are able to see a given chunk. This can be used
/// to broadcast movement updates, for example.
#[derive(Default)]
pub struct Util {
    /// Thread-local bump allocator, reset
    /// every tick.
    ///
    /// This is used to reduce allocation frequency.
    bump: ThreadLocal<Bump>,
    /// The spawner, used to lazily spawn entities.
    spawner: Spawner,
    /// The broadcaster, used to lazily broadcast packets.
    broadcaster: Broadcaster,
}

impl Util {
    /// Returns the thread-local bump allocator.
    pub fn bump(&self) -> &Bump {
        self.bump.get_default()
    }

    /// Allocates a value using the bump allocator
    /// for this thread.
    ///
    /// This is equivalent to `Util::bump().alloc(value)`.
    #[allow(clippy::mut_from_ref)] // This is sound—it just redirects to `bumpalo`.
    pub fn alloc<T>(&self, value: T) -> &mut T {
        self.bump().alloc(value)
    }

    /// Queues an item to be spawned.
    ///
    /// This redirects to `Spawner::spawn`.
    pub fn spawn_item(&self, position: Position, velocity: DVec3, item: ItemStack) {
        self.spawner.spawn_item(position, velocity, item);
    }

    /// This should be called at the end of every tick.
    pub fn reset(&mut self) {
        // Reset bump allocators
        for bump in self.bump.iter_mut() {
            bump.reset();
        }
    }

    /// Broadcasts a packet to all players who
    /// are able to see a given entity.
    ///
    /// The packet is sent lazily in a separate system.
    ///
    /// If `neq` is set to an entity, the packet
    /// will not be sent to that player.
    ///
    /// This function runs in linear time with
    /// regard to the number of players able to see
    /// the entity.
    pub fn broadcast_entity_update<P>(&self, entity: Entity, packet: P, neq: Option<Entity>)
    where
        P: Packet + Clone + 'static,
    {
        self.broadcaster
            .broadcast_entity_update(entity, packet, neq);
    }

    /// Broadcasts a packet to all players who
    /// are able to see a given chunk.
    ///
    /// The packet is sent lazily in a separate system.
    ///
    /// If `neq` is set to an entity, the packet
    /// will not be sent to that player.
    ///
    /// This function runs in linear time with
    /// regard to the number of players able to see
    /// the chunk.
    pub fn broadcast_chunk_update<P>(&self, chunk: ChunkPosition, packet: P, neq: Option<Entity>)
    where
        P: Packet + Clone + 'static,
    {
        self.broadcaster.broadcast_chunk_update(chunk, packet, neq);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_logic::ChunkHolders;
    use crate::testframework as t;
    use crate::util::broadcaster::BroadcasterSystem;
    use feather_core::network::packet::implementation::EntityHeadLook;
    use feather_core::PacketType;
    use specs::WorldExt;

    #[test]
    fn test_broadcast() {
        let mut chunk_holders = ChunkHolders::default();

        let chunk = ChunkPosition::new(0, 0);
        let other_chunk = ChunkPosition::new(10, 1);

        let (mut world, mut dispatcher) = t::builder().with(BroadcasterSystem, "").build();
        let player1 = t::add_player(&mut world);
        let player2 = t::add_player(&mut world);
        let player3 = t::add_player(&mut world);

        chunk_holders.insert_holder(chunk, player1.entity);
        chunk_holders.insert_holder(chunk, player2.entity);
        chunk_holders.insert_holder(other_chunk, player3.entity);

        let packet = EntityHeadLook::default();

        let util = Util::default();

        util.broadcast_entity_update(player1.entity, packet, Some(player2.entity));

        world.insert(util);
        world.insert(chunk_holders);

        dispatcher.dispatch(&world);
        world.maintain();

        t::assert_packet_received(&player1, PacketType::EntityHeadLook);
        t::assert_packet_not_received(&player2, PacketType::EntityHeadLook);
        t::assert_packet_not_received(&player3, PacketType::EntityHeadLook);
    }
}
