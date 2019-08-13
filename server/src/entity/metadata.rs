//! Definition for entity metadata enum.
use feather_core::{EntityMetadata, Slot};

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
        let mut meta = Metadata::Entity(Entity::new(0, 0, false, false));

        let raw = meta.to_raw_metadata();

        assert_eq!(raw.get(0), Some(MetaEntry::Byte(0)));
        assert_eq!(raw.get(5), Some(MetaEntry::Boolean(false)));
        assert_eq!(raw.get(6), None);
    }
}
