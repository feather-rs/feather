//! Dealing with entities, including associated components and events.
//! Submodules here are implementations of specific entities, such as items,
//! block entities, monsters, etc. Player entities are handled in `crate::player`,
//! not here.

pub mod arrow;
pub mod falling_block;
pub mod item;

use crate::game::Game;
use feather_core::entity::EntityData;
use feather_core::{Packet, Position};
use fecs::{EntityBuilder, EntityRef, IntoQuery, Read, World, Write};
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

/// The velocity of an entity on the previous tick.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PreviousVelocity(pub glm::DVec3);

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

impl Default for PreviousVelocity {
    fn default() -> Self {
        Self(glm::vec3(0.0, 0.0, 0.0))
    }
}

impl Deref for PreviousVelocity {
    type Target = glm::DVec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PreviousVelocity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// The display name of an entity.
///
/// Note that unnamed entities do not have this component.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name(pub String);

/// Position of an entity on the last tick.
///
/// This is updated by `position_reset` system.
#[derive(Debug, Clone, Copy)]
pub struct PreviousPosition(pub Position);

pub trait PacketCreatorFn: Fn(&EntityRef) -> Box<dyn Packet> + Send + Sync + 'static {}
impl<F> PacketCreatorFn for F where F: Fn(&EntityRef) -> Box<dyn Packet> + Send + Sync + 'static {}

/// Component which defines a function returning a packet to send
/// to clients when the entity comes within range. This packet
/// spawns the entity on the client.
pub struct SpawnPacketCreator(pub &'static dyn PacketCreatorFn);

impl SpawnPacketCreator {
    /// Returns the packet to send to clients when the entity is to be
    /// sent to the client.
    pub fn get(&self, accessor: &EntityRef) -> Box<dyn Packet> {
        let f = self.0;

        f(accessor)
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
    pub fn get(&self, accessor: &EntityRef) -> Box<dyn Packet> {
        let f = self.0;

        f(accessor)
    }
}

pub trait ComponentSerializerFn:
    Fn(&Game, &EntityRef) -> EntityData + Send + Sync + 'static
{
}

impl<F> ComponentSerializerFn for F where
    F: Fn(&Game, &EntityRef) -> EntityData + Send + Sync + 'static
{
}

/// Component which stores a function needed to convert an entity's
/// components to the serializable `EntityData`.
pub struct ComponentSerializer(pub &'static dyn ComponentSerializerFn);

impl ComponentSerializer {
    pub fn serialize(&self, game: &Game, accessor: &EntityRef) -> EntityData {
        let f = self.0;

        f(game, accessor)
    }
}

#[system]
pub fn previous_position_velocity_reset(world: &mut World) {
    <(Read<Position>, Write<PreviousPosition>)>::query().par_for_each_mut(
        world.inner_mut(),
        |(pos, mut previous_pos)| {
            previous_pos.0 = *pos;
        },
    );
    <(Read<Velocity>, Write<PreviousVelocity>)>::query().par_for_each_mut(
        world.inner_mut(),
        |(vel, mut previous_vel)| {
            previous_vel.0 = vel.0;
        },
    );
}

/// Inserts the base components for an entity into an `EntityBuilder`.
///
/// This currently includes:
/// * Velocity (0) and PreviousVelocity
/// * Entity ID for the protocol
pub fn base() -> EntityBuilder {
    let id = new_id();
    EntityBuilder::new()
        .with(EntityId(id))
        .with(Velocity::default())
        .with(PreviousVelocity::default())
        .with(PreviousPosition(position!(0.0, 0.0, 0.0)))
}

/// Returns a new entity ID.
pub fn new_id() -> i32 {
    ENTITY_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}
