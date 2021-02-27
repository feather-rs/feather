//! Components which involve serializing entities,
//! be it over the network or onto the disk.

use crate::Game;
use feather_core::anvil::{block_entity::BlockEntityData, entity::EntityData};
use feather_core::network::Packet;
use fecs::EntityRef;

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

pub trait BlockSerializerFn:
    Fn(&Game, &EntityRef) -> BlockEntityData + Send + Sync + 'static
{
}

impl<F> BlockSerializerFn for F where
    F: Fn(&Game, &EntityRef) -> BlockEntityData + Send + Sync + 'static
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

/// Similar to `ComponentSerializer`, but for block entities (e.g. chests).
pub struct BlockSerializer(pub &'static dyn BlockSerializerFn);

impl BlockSerializer {
    pub fn serialize(&self, game: &Game, accessor: &EntityRef) -> BlockEntityData {
        let f = self.0;

        f(game, accessor)
    }
}
