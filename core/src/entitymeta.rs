//! This module implements handling of the entity
//! metadata format. See https://wiki.vg/Entity_metadata
//! for the specification.

use crate::bytes_ext::{BytesExt, BytesMutExt, TryGetError};
use crate::network::mctypes::{McTypeRead, McTypeWrite};
use crate::world::BlockPosition;
use crate::Slot;
use bitflags::bitflags;
use bytes::Buf;
use num_traits::FromPrimitive;
use std::collections::BTreeMap;
use uuid::Uuid;

type OptUuid = Option<Uuid>;
type OptChat = Option<String>;

// Meta index constants.
pub const META_INDEX_ENTITY_BITMASK: u8 = 0;
pub const META_INDEX_AIR: u8 = 1;
pub const META_INDEX_CUSTOM_NAME: u8 = 2;
pub const META_INDEX_IS_CUSTOM_NAME_VISIBLE: u8 = 3;
pub const META_INDEX_IS_SILENT: u8 = 4;
pub const META_INDEX_NO_GRAVITY: u8 = 5;

pub const META_INDEX_ITEM_SLOT: u8 = 6;

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
    OptChat(Option<String>),
    Slot(Slot),
    Boolean(bool),
    Rotation(f32, f32, f32),
    Position(BlockPosition),
    OptPosition(Option<BlockPosition>),
    Direction(Direction),
    OptUuid(OptUuid),
    OptBlockId(Option<i32>),
    Nbt(nbt::Blob),
    Particle, // TODO
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
        }
    }
}

pub trait ToMetaEntry {
    fn to_meta_entry(&self) -> MetaEntry;
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

impl ToMetaEntry for bool {
    fn to_meta_entry(&self) -> MetaEntry {
        MetaEntry::Boolean(*self)
    }
}

impl ToMetaEntry for Slot {
    fn to_meta_entry(&self) -> MetaEntry {
        MetaEntry::Slot(*self)
    }
}

impl ToMetaEntry for f32 {
    fn to_meta_entry(&self) -> MetaEntry {
        MetaEntry::Float(*self)
    }
}

impl ToMetaEntry for OptUuid {
    fn to_meta_entry(&self) -> MetaEntry {
        MetaEntry::OptUuid(*self)
    }
}

impl ToMetaEntry for BlockPosition {
    fn to_meta_entry(&self) -> MetaEntry {
        MetaEntry::Position(*self)
    }
}

impl ToMetaEntry for OptChat {
    fn to_meta_entry(&self) -> MetaEntry {
        MetaEntry::OptChat(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct EntityMetadata {
    values: BTreeMap<u8, MetaEntry>,
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
        //.with(META_INDEX_ENTITY_BITMASK, EntityBitMask::empty().bits())
        //.with(META_INDEX_AIR, 0i32)
        //.with(META_INDEX_CUSTOM_NAME, OptChat::None)
        //.with(META_INDEX_IS_CUSTOM_NAME_VISIBLE, false)
        //.with(META_INDEX_IS_SILENT, false)
        //.with(META_INDEX_NO_GRAVITY, false)
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
}

impl Default for EntityMetadata {
    fn default() -> Self {
        Self::new()
    }
}

pub trait EntityMetaWrite {
    fn push_metadata(&mut self, meta: &EntityMetadata);
}

pub trait EntityMetaRead {
    fn try_get_metadata(&mut self) -> anyhow::Result<EntityMetadata>;
}

impl<B> EntityMetaWrite for B
where
    B: BytesMutExt + McTypeWrite,
{
    fn push_metadata(&mut self, meta: &EntityMetadata) {
        for (index, entry) in meta.values.iter() {
            self.push_u8(*index);
            self.push_var_int(entry.id());
            write_entry_to_buf(entry, self);
        }

        self.push_u8(0xff); // End of metadata
    }
}

impl<B> EntityMetaRead for B
where
    B: Buf + std::io::Read,
{
    fn try_get_metadata(&mut self) -> anyhow::Result<EntityMetadata> {
        let mut values = BTreeMap::new();

        while self.has_remaining() {
            let index = self.try_get_u8()?;

            if index == 0xFF {
                break;
            }

            let entry = try_get_entry(self)?;
            values.insert(index, entry);
        }

        Ok(EntityMetadata { values })
    }
}

fn write_entry_to_buf<B>(entry: &MetaEntry, buf: &mut B)
where
    B: BytesMutExt + McTypeWrite,
{
    match entry {
        MetaEntry::Byte(x) => buf.push_i8(*x),
        MetaEntry::VarInt(x) => {
            buf.push_var_int(*x);
        }
        MetaEntry::Float(x) => buf.push_f32(*x),
        MetaEntry::String(x) => buf.push_string(x),
        MetaEntry::Chat(x) => buf.push_string(x),
        MetaEntry::OptChat(ox) => {
            if let Some(x) = ox {
                buf.push_bool(true);
                buf.push_string(x);
            } else {
                buf.push_bool(false);
            }
        }
        MetaEntry::Slot(slot) => {
            buf.push_slot(*slot);
        }
        MetaEntry::Boolean(x) => buf.push_bool(*x),
        MetaEntry::Rotation(x, y, z) => {
            buf.push_f32(*x);
            buf.push_f32(*y);
            buf.push_f32(*z);
        }
        MetaEntry::Position(x) => buf.push_position(x),
        MetaEntry::OptPosition(ox) => {
            if let Some(x) = ox {
                buf.push_bool(true);
                buf.push_position(x);
            } else {
                buf.push_bool(false);
            }
        }
        MetaEntry::Direction(x) => {
            buf.push_var_int(x.id());
        }
        MetaEntry::OptUuid(ox) => {
            if let Some(x) = ox {
                buf.push_bool(true);
                buf.push_uuid(x);
            } else {
                buf.push_bool(false);
            }
        }
        MetaEntry::OptBlockId(ox) => {
            if let Some(x) = ox {
                buf.push_var_int(*x);
            } else {
                buf.push_var_int(0); // No value implies air
            }
        }
        MetaEntry::Nbt(val) => buf.push_nbt(val),
        MetaEntry::Particle => unimplemented!(),
    }
}

fn try_get_entry<B>(buf: &mut B) -> anyhow::Result<MetaEntry>
where
    B: Buf + McTypeRead,
{
    let id = buf.try_get_var_int()?;

    Ok(match id {
        0 => MetaEntry::Byte(buf.try_get_i8()?),
        1 => MetaEntry::VarInt(buf.try_get_var_int()?),
        2 => MetaEntry::Float(buf.try_get_f32()?),
        3 => MetaEntry::String(buf.try_get_string()?),
        4 => MetaEntry::Chat(buf.try_get_string()?),
        5 => MetaEntry::OptChat(if buf.try_get_bool()? {
            Some(buf.try_get_string()?)
        } else {
            None
        }),
        6 => MetaEntry::Slot(buf.try_get_slot()?),
        7 => MetaEntry::Boolean(buf.try_get_bool()?),
        8 => MetaEntry::Rotation(buf.try_get_f32()?, buf.try_get_f32()?, buf.try_get_f32()?),
        9 => MetaEntry::Position(buf.try_get_position()?),
        10 => MetaEntry::OptPosition(if buf.try_get_bool()? {
            Some(buf.try_get_position()?)
        } else {
            None
        }),
        11 => MetaEntry::Direction(
            Direction::from_i32(buf.try_get_var_int()?).ok_or(TryGetError::InvalidValue(0))?,
        ),
        12 => MetaEntry::OptUuid(if buf.try_get_bool()? {
            Some(buf.try_get_uuid()?)
        } else {
            None
        }),
        13 => MetaEntry::OptBlockId(if buf.try_get_bool()? {
            Some(buf.try_get_var_int()?)
        } else {
            None
        }),
        14 => MetaEntry::Nbt(buf.try_get_nbt()?),
        x => return Err(TryGetError::InvalidValue(x).into()),
    })
}

#[derive(Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum Direction {
    Down,
    Up,
    North,
    South,
    West,
    East,
}

impl Direction {
    pub fn id(&self) -> i32 {
        match self {
            Direction::Down => 0,
            Direction::Up => 1,
            Direction::North => 2,
            Direction::South => 3,
            Direction::West => 4,
            Direction::East => 5,
        }
    }
}
