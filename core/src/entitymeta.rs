//! This module implements handling of the entity
//! metadata format. See https://wiki.vg/Entity_metadata
//! for the specification.

use crate::bytebuf::{BufMutAlloc, ByteBuf};
use crate::network::mctypes::McTypeWrite;
use crate::world::BlockPosition;
use hashbrown::HashMap;
use uuid::Uuid;

pub enum MetaEntry {
    Byte(i8),
    VarInt(i32),
    Float(f32),
    String(String),
    Chat(String),
    OptChat(Option<String>),
    Slot, // TODO
    Boolean(bool),
    Rotation(f32, f32, f32),
    Position(BlockPosition),
    OptPosition(Option<BlockPosition>),
    Direction(Direction),
    OptUuid(Option<Uuid>),
    OptBlockId(Option<i32>),
    Nbt,      // TODO
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
            MetaEntry::Slot => 6,
            MetaEntry::Boolean(_) => 7,
            MetaEntry::Rotation(_, _, _) => 8,
            MetaEntry::Position(_) => 9,
            MetaEntry::OptPosition(_) => 10,
            MetaEntry::Direction(_) => 11,
            MetaEntry::OptUuid(_) => 12,
            MetaEntry::OptBlockId(_) => 13,
            MetaEntry::Nbt => 14,
            MetaEntry::Particle => 15,
        }
    }
}

pub struct EntityMetadata {
    values: HashMap<u8, MetaEntry>,
}

pub trait EntityMetaIo {
    fn write_metadata(&mut self, meta: EntityMetadata);
    fn read_metadata(&mut self) -> EntityMetadata;
}

impl EntityMetaIo for ByteBuf {
    fn write_metadata(&mut self, meta: EntityMetadata) {
        for (index, entry) in meta.values {
            self.write_u8(index);
            self.write_var_int(entry.id());
            write_entry_to_buf(entry, self);
        }

        self.write_u8(0xff); // End of metadata
    }

    fn read_metadata(&mut self) -> EntityMetadata {
        unimplemented!()
    }
}

fn write_entry_to_buf(entry: MetaEntry, buf: &mut ByteBuf) {
    match entry {
        MetaEntry::Byte(x) => buf.write_i8(x),
        MetaEntry::VarInt(x) => buf.write_var_int(x),
        MetaEntry::Float(x) => buf.write_f32_be(x),
        MetaEntry::String(x) => buf.write_string(&x),
        MetaEntry::Chat(x) => buf.write_string(&x),
        MetaEntry::OptChat(ox) => {
            if let Some(x) = ox {
                buf.write_bool(true);
                buf.write_string(&x);
            } else {
                buf.write_bool(false);
            }
        }
        MetaEntry::Slot => unimplemented!(),
        MetaEntry::Boolean(x) => buf.write_bool(x),
        MetaEntry::Rotation(x, y, z) => {
            buf.write_f32_be(x);
            buf.write_f32_be(y);
            buf.write_f32_be(z);
        }
        MetaEntry::Position(x) => buf.write_position(&x),
        MetaEntry::OptPosition(ox) => {
            if let Some(x) = ox {
                buf.write_bool(true);
                buf.write_position(&x);
            } else {
                buf.write_bool(false);
            }
        }
        MetaEntry::Direction(x) => buf.write_var_int(x.id()),
        MetaEntry::OptUuid(ox) => {
            if let Some(x) = ox {
                buf.write_bool(true);
                buf.write_uuid(&x);
            } else {
                buf.write_bool(false);
            }
        }
        MetaEntry::OptBlockId(ox) => {
            if let Some(x) = ox {
                buf.write_var_int(x);
            } else {
                buf.write_var_int(0); // No value implies air
            }
        }
        MetaEntry::Nbt => unimplemented!(),
        MetaEntry::Particle => unimplemented!(),
    }
}

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
