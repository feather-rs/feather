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
