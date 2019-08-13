//! Definition for entity metadata enum.

#![allow(clippy::too_many_arguments)] // TODO: builder patterm

use feather_core::{EntityMetadata, Slot};
use specs::{Component, VecStorage};

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
    type Storage = VecStorage<Self>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use feather_core::entitymeta::MetaEntry;

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
}
