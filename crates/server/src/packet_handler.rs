//! Handler functions for packets.
//!
//! Packet handlers are stored in the `PacketHandlerRegistry`.
//! Each time a packet is received, the handler
//! for that packet is invoked.

use ahash::AHashMap;
use base::State;
use ecs::{Entity, SysResult};
use once_cell::sync::Lazy;
use protocol::{ClientPlayPacket, VariantOf};

type DynPacketHandler =
    Box<dyn Fn(&mut State, Entity, ClientPlayPacket) -> SysResult + Send + Sync + 'static>;

#[derive(Default)]
pub struct PacketHandlerRegistry {
    /// Maps `ClientPlayPacket` IDs to handlers for those packets.
    handlers: AHashMap<u32, DynPacketHandler>,
}

impl PacketHandlerRegistry {
    fn new() -> Self {
        Self::default()
    }

    /// Adds a packet handler for packets of type `T`.
    ///
    /// The handler function takes as parameters:
    /// * The `State`
    /// * The player who sent the packet
    /// * The packet itself
    ///
    /// Like systems, a packet handler returns a `SysResult`.
    fn with<T>(mut self, handler: fn(&mut State, Entity, T) -> SysResult) -> Self
    where
        T: VariantOf<ClientPlayPacket> + 'static,
    {
        let f: DynPacketHandler = Box::new(move |state, player, packet| {
            let packet =
                T::destructure(packet).expect("packet handler called with mismatched packet");
            handler(state, player, packet)
        });
        if self.handlers.insert(T::discriminant_id(), f).is_some() {
            panic!(
                "duplicate packet handlers for packet type {}",
                std::any::type_name::<T>()
            );
        }

        self
    }

    /// Handles a packet by invoking its handler.
    pub fn handle(&self, state: &mut State, entity: Entity, packet: ClientPlayPacket) -> SysResult {
        match self.handlers.get(&packet.id()) {
            Some(handler) => handler(state, entity, packet),
            None => Ok(()),
        }
    }
}

// Register packet handlers here.
pub static PACKET_HANDLERS: Lazy<PacketHandlerRegistry> =
    Lazy::new(|| PacketHandlerRegistry::new());
