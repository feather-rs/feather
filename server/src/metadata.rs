//! Entity metadata implementation.

use feather_core::entitymeta::EntityMetadata;
use feather_core::inventory::Slot;
use feather_core::world::BlockPosition;
use uuid::Uuid;

type OptUuid = Option<Uuid>;

bitflags! {
    pub struct EntityBitMask: u8 {
        const ON_FIRE = 0x01;
        const CROUCHED = 0x02;
        const SPRINTING = 0x08;
        const SWIMMING = 0x10;
        const INVISIBLE = 0x20;
        const GLOWING_EFFECT = 0x40;
        const FLYING_WITH_ELYTRA = 0x80;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct ArrowBitMask: u8 {
        const CRITICAL = 0x01;
        const NO_CLIP = 0x02;
    }
}

lazy_static! {
    pub static ref EMPTY_METADATA: Metadata = { Metadata::Entity(Entity::default()) };
}

pub type Metadata = _Metadata;

entity_metadata! {
    _Metadata,
    Entity {
        bit_mask: u8() = 0,
        air: VarInt(300) = 1,
        silent: bool() = 4,
        no_gravity: bool() = 5,
    },
    Item: Entity {
        item: Slot() = 6,
    },
    Living: Entity {
        hand_states: u8() = 6,
        health: f32(1.0) = 7,
        potion_effect_color: VarInt() = 8,
        potion_effect_ambient: bool() = 9,
        arrows: VarInt() = 10,
    },
    Player: Living {
        additional_hearts: f32() = 11,
        score: VarInt() = 12,
        displayed_skin_parts: u8() = 13,
        main_hand: u8(1) = 14,
    },
    Arrow: Entity {
        arrow_bit_mask: u8() = 6,
        shooter: OptUuid() = 7,
    },
    TippedArrow: Arrow {
        color: VarInt() = 8,
    },
    FallingBlock: Entity {
        spawn_position: BlockPosition() = 6,
    },
}
