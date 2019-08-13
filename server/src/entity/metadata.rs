//! Definition for entity metadata enum.
use feather_core::{EntityMetadata, Slot};

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
}
