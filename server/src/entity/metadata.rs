//! Definition for entity metadata enum.

#![allow(clippy::too_many_arguments)] // TODO: builder patterm

use crate::util::Util;
use feather_core::packet::PacketEntityMetadata;
use feather_core::{EntityMetadata, Slot};
use specs::storage::ComponentEvent;
use specs::{
    BitSet, Component, Entities, FlaggedStorage, Join, Read, ReaderId, System, VecStorage,
    WriteStorage,
};

bitflags! {
    pub struct EntityBitMask: u8 {
        const ON_FIRE = 0x01;
        const CROUCHED = 0x02;
        const SPRITING = 0x08;
        const SWIMMING = 0x10;
        const INVISIBLE = 0x20;
        const GLOWING_EFFECT = 0x40;
        const FLYING_WITH_ELYTRA = 0x80;
    }
}

entity_metadata! {
    Metadata,
    Entity {
        bit_mask: u8 = 0,
        air: VarInt = 1,
        silent: bool = 4,
        no_gravity: bool = 5,
    },
    Item: Entity {
        item: Slot = 6,
    },
    Living: Entity {
        hand_states: u8 = 6,
        health: f32 = 7,
        potion_effect_color: VarInt = 8,
        potion_effect_ambient: bool = 9,
        arrows: VarInt = 10,
    },
    Player: Living {
        additional_hearts: f32 = 11,
        score: VarInt = 12,
        displayed_skin_parts: u8 = 13,
        main_hand: u8 = 14,
    },
}

impl Component for Metadata {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

/// System for broadcasting entity metadata updates.
#[derive(Default)]
pub struct MetadataBroadcastSystem {
    dirty: BitSet,
    reader: Option<ReaderId<ComponentEvent>>,
}

impl<'a> System<'a> for MetadataBroadcastSystem {
    type SystemData = (WriteStorage<'a, Metadata>, Read<'a, Util>, Entities<'a>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut metadatas, util, entities) = data;

        self.dirty.clear();

        read_flagged_events!(metadatas, self.reader, self.dirty);

        // Ensure that metadata update events are not
        // triggered for this mutation of the storage.
        metadatas.set_event_emission(false);

        // Go through updated metadata and broadcast changes.
        for (metadata, entity, _) in (&mut metadatas, &entities, &self.dirty).join() {
            let packet = PacketEntityMetadata {
                entity_id: entity.id() as i32,
                metadata: metadata.to_raw_metadata(),
            };

            util.broadcast_entity_update(entity, packet, None);
        }

        metadatas.set_event_emission(true);
    }

    flagged_setup_impl!(Metadata, reader);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::EntityType;
    use crate::testframework as t;
    use feather_core::entitymeta::MetaEntry;
    use feather_core::network::cast_packet;
    use feather_core::PacketType;
    use specs::WorldExt;

    #[test]
    fn test_basic() {
        let mut meta = Metadata::Entity(Entity::new(
            (EntityBitMask::ON_FIRE | EntityBitMask::CROUCHED).bits(),
            0,
            false,
            false,
        ));

        let raw = meta.to_raw_metadata();

        assert_eq!(raw.get(0), Some(MetaEntry::Byte(0b0000_0011)));
        assert_eq!(raw.get(5), Some(MetaEntry::Boolean(false)));
        assert_eq!(raw.get(6), None);
    }

    #[test]
    fn test_inheritance() {
        let _meta = Metadata::Item(Item::new(
            (EntityBitMask::ON_FIRE).bits(),
            0,
            false,
            false,
            None,
        ));
    }

    #[test]
    fn test_metadata_update_system() {
        let (mut w, mut d) = t::builder()
            .with(MetadataBroadcastSystem::default(), "")
            .build();

        // Metadata is inserted here, which causes update event
        let entity = t::add_entity(&mut w, EntityType::Test, true);
        let player = t::add_player(&mut w);

        d.dispatch(&w);
        w.maintain();

        // Ensure that packet was sent
        let packet = t::assert_packet_received(&player, PacketType::EntityMetadata);
        let packet = cast_packet::<PacketEntityMetadata>(&*packet);

        assert_eq!(packet.entity_id, entity.id() as i32);
    }
}
