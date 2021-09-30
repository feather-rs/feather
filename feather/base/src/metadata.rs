//! This module implements the entity
//! metadata format. See <https://wiki.vg/Entity_metadata>
//! for the specification.

use crate::{Direction, ValidBlockPosition};
use bitflags::bitflags;
use libcraft_items::InventorySlot;
use std::collections::BTreeMap;
use uuid::Uuid;

pub type OptUuid = Option<Uuid>;
pub type OptChat = Option<String>;
pub type OptVarInt = Option<i32>;

// Meta index constants.
pub const META_INDEX_ENTITY_BITMASK: u8 = 0;
pub const META_INDEX_AIR: u8 = 1;
pub const META_INDEX_CUSTOM_NAME: u8 = 2;
pub const META_INDEX_IS_CUSTOM_NAME_VISIBLE: u8 = 3;
pub const META_INDEX_IS_SILENT: u8 = 4;
pub const META_INDEX_NO_GRAVITY: u8 = 5;

pub const META_INDEX_POSE: u8 = 6;

pub const META_INDEX_FALLING_BLOCK_SPAWN_POSITION: u8 = 7;

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

#[derive(Clone, Debug, PartialEq)]
pub enum MetaEntry {
    Byte(i8),
    VarInt(i32),
    Float(f32),
    String(String),
    Chat(String),
    OptChat(OptChat),
    Slot(InventorySlot),
    Boolean(bool),
    Rotation(f32, f32, f32),
    Position(ValidBlockPosition),
    OptPosition(Option<ValidBlockPosition>),
    Direction(Direction),
    OptUuid(OptUuid),
    OptBlockId(Option<i32>),
    Nbt(nbt::Blob),
    Particle,
    VillagerData(i32, i32, i32),
    OptVarInt(OptVarInt),
    Pose(i32),
}

impl MetaEntry {
    pub fn id(&self) -> i32 {
        match self {
            MetaEntry::Byte(_) => 0,
            MetaEntry::VarInt(_) => 1,
            MetaEntry::Float(_) => 2,
            MetaEntry::String(_) => 3,
            MetaEntry::Chat(_) => 4,
            MetaEntry::OptChat(_) => 5,
            MetaEntry::Slot(_) => 6,
            MetaEntry::Boolean(_) => 7,
            MetaEntry::Rotation(_, _, _) => 8,
            MetaEntry::Position(_) => 9,
            MetaEntry::OptPosition(_) => 10,
            MetaEntry::Direction(_) => 11,
            MetaEntry::OptUuid(_) => 12,
            MetaEntry::OptBlockId(_) => 13,
            MetaEntry::Nbt(_) => 14,
            MetaEntry::Particle => 15,
            MetaEntry::VillagerData(_, _, _) => 16,
            MetaEntry::OptVarInt(_) => 17,
            MetaEntry::Pose(_) => 18,
        }
    }
}

pub enum Pose {
    Standing,
    FallFlying,
    Sleeping,
    Swimming,
    SpinAttack,
    Sneaking,
    Dying,
}

impl ToMetaEntry for Pose {
    fn to_meta_entry(&self) -> MetaEntry {
        MetaEntry::Pose(match self {
            Self::Standing => 0,
            Self::FallFlying => 1,
            Self::Sleeping => 2,
            Self::Swimming => 3,
            Self::SpinAttack => 4,
            Self::Sneaking => 5,
            Self::Dying => 6,
        })
    }
}

pub trait ToMetaEntry {
    fn to_meta_entry(&self) -> MetaEntry;
}

impl ToMetaEntry for u8 {
    fn to_meta_entry(&self) -> MetaEntry {
        MetaEntry::Byte(*self as i8)
    }
}

impl ToMetaEntry for i8 {
    fn to_meta_entry(&self) -> MetaEntry {
        MetaEntry::Byte(*self)
    }
}

impl ToMetaEntry for i32 {
    fn to_meta_entry(&self) -> MetaEntry {
        MetaEntry::VarInt(*self)
    }
}

impl ToMetaEntry for f32 {
    fn to_meta_entry(&self) -> MetaEntry {
        MetaEntry::Float(*self)
    }
}

impl ToMetaEntry for OptChat {
    fn to_meta_entry(&self) -> MetaEntry {
        MetaEntry::OptChat(self.clone())
    }
}

impl ToMetaEntry for InventorySlot {
    fn to_meta_entry(&self) -> MetaEntry {
        MetaEntry::Slot(self.clone())
    }
}

impl ToMetaEntry for bool {
    fn to_meta_entry(&self) -> MetaEntry {
        MetaEntry::Boolean(*self)
    }
}

impl ToMetaEntry for ValidBlockPosition {
    fn to_meta_entry(&self) -> MetaEntry {
        MetaEntry::Position(*self)
    }
}

impl ToMetaEntry for OptUuid {
    fn to_meta_entry(&self) -> MetaEntry {
        MetaEntry::OptUuid(*self)
    }
}

impl ToMetaEntry for OptVarInt {
    fn to_meta_entry(&self) -> MetaEntry {
        MetaEntry::OptVarInt(*self)
    }
}

#[derive(Clone, Debug)]
pub struct EntityMetadata {
    pub values: BTreeMap<u8, MetaEntry>,
}

impl EntityMetadata {
    pub fn new() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }

    /// Returns an entity metadata with the defaults for an `Entity`.
    pub fn entity_base() -> Self {
        Self::new()
            .with(META_INDEX_ENTITY_BITMASK, EntityBitMask::empty().bits())
            .with(META_INDEX_AIR, 0i32)
            .with(META_INDEX_CUSTOM_NAME, OptChat::None)
            .with(META_INDEX_IS_CUSTOM_NAME_VISIBLE, false)
            .with(META_INDEX_IS_SILENT, false)
            .with(META_INDEX_NO_GRAVITY, false)
    }

    pub fn with_many(mut self, values: &[(u8, MetaEntry)]) -> Self {
        for val in values {
            self.values.insert(val.0, val.1.clone());
        }

        self
    }

    pub fn set(&mut self, index: u8, entry: impl ToMetaEntry) {
        self.values.insert(index, entry.to_meta_entry());
    }

    pub fn with(mut self, index: u8, entry: impl ToMetaEntry) -> Self {
        self.set(index, entry);
        self
    }

    pub fn get(&self, index: u8) -> Option<MetaEntry> {
        self.values.get(&index).cloned()
    }

    pub fn iter(&self) -> impl Iterator<Item = (u8, &MetaEntry)> {
        self.values.iter().map(|(key, entry)| (*key, entry))
    }
}

impl Default for EntityMetadata {
    fn default() -> Self {
        Self::new()
    }
}
