//! Dealing with entities, including associated components and events.
//! Submodules here are implementations of specific entities, such as items,
//! block entities, monsters, etc. Player entities are handled in `crate::player`,
//! not here.

// pub mod item;

use feather_core::Position;
use fecs::EntityBuilder;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicI32, Ordering};

/// ID of an entity. This value is generally unique.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct EntityId(pub i32);

/// Entity ID counter, used to create new entity IDs.
pub static ENTITY_ID_COUNTER: AtomicI32 = AtomicI32::new(0);

/// The velocity of an entity.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Velocity(pub glm::DVec3);

impl Default for Velocity {
    fn default() -> Self {
        Self(glm::vec3(0.0, 0.0, 0.0))
    }
}

impl Deref for Velocity {
    type Target = glm::DVec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Velocity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// The display name of an entity.
///
/// Note that unnamed entities do not have this component.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name(pub String);

/*
/// Position of an entity on the last tick.
///
/// This is updated by `position_reset` system.
#[derive(Debug, Clone, Copy)]
pub struct PreviousPosition(pub Position);

pub trait PacketCreatorFn:
    Fn(&EntityAccessor, &PreparedWorld) -> Box<dyn Packet> + Send + Sync + 'static
{
}
impl<F> PacketCreatorFn for F where
    F: Fn(&EntityAccessor, &PreparedWorld) -> Box<dyn Packet> + Send + Sync + 'static
{
}

/// Component which defines a function returning a packet to send
/// to clients when the entity comes within range. This packet
/// spawns the entity on the client.
pub struct SpawnPacketCreator(pub &'static dyn PacketCreatorFn);

impl SpawnPacketCreator {
    /// Returns the packet to send to clients when the entity is to be
    /// sent to the client.
    pub fn get(&self, accessor: &EntityAccessor, world: &PreparedWorld) -> Box<dyn Packet> {
        let f = self.0;

        f(accessor, world)
    }
}

/// Component which defines a function returning a packet to send
/// to _all_ clients when the entity is created or the client joins.
/// This packet is sent before that returned by `SpawnPacketCreator`,
/// and it differs in that the packet is broadcasted globally
/// rather than to nearby clients.
///
/// Another difference is that the packet from `SpawnPacketCreator` is not sent
/// to its own entity, while that from `CreationPacketCreator` is.
///
/// An example of a use case for this packet is the `PlayerInfo` packet
/// sent when a player joins—it is sent to all players, not just those
/// that are able to see the player.
pub struct CreationPacketCreator(pub &'static dyn PacketCreatorFn);

impl CreationPacketCreator {
    /// Returns the packet to send to clients when the entity is created.
    pub fn get(&self, accessor: &EntityAccessor, world: &PreparedWorld) -> Box<dyn Packet> {
        let f = self.0;

        f(accessor, world)
    }
}

#[event_handler]
pub fn position_reset(
    events: &[EntityMoveEvent],
    _query: &mut Query<(Read<Position>, Write<PreviousPosition>)>,
    world: &mut PreparedWorld,
) {
    events.iter().for_each(|event| {
        let pos = *world.get_component::<Position>(event.entity).unwrap();
        let mut prev_pos = world
            .get_component_mut::<PreviousPosition>(event.entity)
            .unwrap();

        prev_pos.0 = pos;
    });
}
*/

/// Inserts the base components for an entity into an `EntityBuilder`.
///
/// This currently includes:
/// * Velocity (0)
/// * Entity ID
/// * Position and previous position
/// * Triggers `EntityCreateEvent`
pub fn base(position: Position) -> EntityBuilder {
    let id = new_id();
    EntityBuilder::new()
        .with(EntityId(id))
        .with(position)
        //.with(PreviousPosition(position))
        .with(Velocity::default())
}

/// Returns a new entity ID.
pub fn new_id() -> i32 {
    ENTITY_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}
