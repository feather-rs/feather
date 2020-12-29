use crate::bytes_ext::{BytesExt, BytesMutExt, TryGetError};
use bytes::{Buf, BytesMut};
use feather_anvil::entity::ItemNbt;
use feather_entity_metadata::{EntityMetadata, MetaEntry};
use feather_items::{Item, ItemStack};
use feather_util::BlockPosition;
use feather_util::Direction;
use num_traits::FromPrimitive;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::BTreeMap;
use std::io::Read;
use uuid::Uuid;

/// Identifies a type to which Minecraft-specific
/// types (`VarInt`, `VarLong`, etc.) can be written.
pub trait McTypeWrite {
    /// Writes a `VarInt` to the object. See wiki.vg for
    /// details on `VarInt`s and related types.
    ///
    /// Returns the number of bytes used to encode this integer.
    fn push_var_int(&mut self, x: i32) -> usize;
    /// Writes a string to the object. This method
    /// will first write the length of the string in bytes
    /// encodes as a `VarInt` and will then write
    /// the UTF-8 bytes of the string.
    fn push_string(&mut self, x: &str);

    fn push_position(&mut self, x: &BlockPosition);

    fn push_bool(&mut self, x: bool);

    fn push_uuid(&mut self, x: &Uuid);

    fn push_nbt<T: Serialize>(&mut self, x: &T);

    fn push_slot(&mut self, slot: Option<ItemStack>);
}

/// Identifies a type from which Minecraft-specified
/// types can be read.
pub trait McTypeRead {
    /// Reads a `VarInt` from this object, returning
    /// `Some(x)` if successful or `None` if the object
    /// does not contain a valid `VarInt`.
    fn try_get_var_int(&mut self) -> Result<i32, TryGetError>;
    /// Reads a string from the object.
    fn try_get_string(&mut self) -> Result<String, TryGetError>;

    fn try_get_position(&mut self) -> Result<BlockPosition, TryGetError>;

    fn try_get_bool(&mut self) -> Result<bool, TryGetError>;

    fn try_get_uuid(&mut self) -> anyhow::Result<Uuid>;

    fn try_get_nbt<T: DeserializeOwned>(&mut self) -> Result<T, nbt::Error>;

    fn try_get_slot(&mut self) -> Result<Option<ItemStack>, TryGetError>;
}

impl McTypeWrite for BytesMut {
    fn push_var_int(&mut self, mut x: i32) -> usize {
        let mut bytes_written = 0;
        loop {
            let mut temp = (x & 0b0111_1111) as u8;
            x = (x >> 7) & (i32::max_value() >> 6);
            if x != 0 {
                temp |= 0b1000_0000;
            }
            self.push_u8(temp);
            bytes_written += 1;
            if x == 0 {
                break;
            }
        }

        bytes_written
    }

    /// Writes a string to the object. This method
    /// will first write the length of the string in bytes
    /// encodes as a `VarInt` and will then write
    /// the UTF-8 bytes of the string.
    fn push_string(&mut self, x: &str) {
        let bytes = x.as_bytes();
        self.push_var_int(bytes.len() as i32);

        self.extend_from_slice(bytes);
    }

    fn push_position(&mut self, x: &BlockPosition) {
        let result: u64 = ((x.x as u64 & 0x03FF_FFFF) << 38)
            | ((x.y as u64 & 0xFFF) << 26)
            | (x.z as u64 & 0x03FF_FFFF);

        self.push_u64(result);
    }

    fn push_bool(&mut self, x: bool) {
        if x {
            self.push_u8(1);
        } else {
            self.push_u8(0);
        }
    }

    fn push_uuid(&mut self, x: &Uuid) {
        self.extend_from_slice(&x.as_bytes()[..]);
    }

    fn push_nbt<T: Serialize>(&mut self, val: &T) {
        // TODO: fix inefficient use of temp buf.
        let mut temp = vec![];
        nbt::to_writer(&mut temp, val, None).unwrap(); // Unwrap is safe because writing would only fail if a struct couldn't be written
        self.extend_from_slice(&temp);
    }

    fn push_slot(&mut self, slot: Option<ItemStack>) {
        self.push_bool(slot.is_some());

        if let Some(slot) = slot.as_ref() {
            self.push_var_int(slot.ty.vanilla_id() as i32);
            self.push_i8(slot.amount as i8);
            let tags: ItemNbt = slot.into();

            if tags != Default::default() {
                self.push_nbt(&tags);
            } else {
                self.push_i8(0x00); // TAG_End
            }
        }
    }
}

impl<B: Buf + Read> McTypeRead for B {
    /// Reads a `VarInt` from this object, returning
    /// `Some(x)` if successful or `None` if the object
    /// does not contain a valid `VarInt`.
    fn try_get_var_int(&mut self) -> Result<i32, TryGetError> {
        let mut num_read = 0;
        let mut result = 0;
        loop {
            if self.remaining() == 0 {
                return Err(TryGetError::NotEnoughBytes);
            }
            let read = self.try_get_u8()?;
            let value = i32::from(read & 0b0111_1111);
            result |= value.overflowing_shl(7u32 * num_read).0;

            num_read += 1;
            if num_read > 5 {
                return Err(TryGetError::NotEnoughBytes);
            }
            if read & 0b1000_0000 == 0 {
                break;
            }
        }
        Ok(result)
    }

    /// Reads a string from the object.
    fn try_get_string(&mut self) -> Result<String, TryGetError> {
        let len = self.try_get_var_int();
        if let Ok(len) = len {
            // Check that the client isn't trying
            // to make the server allocate ridiculous
            // amounts of memory
            if len > 32767 {
                return Err(TryGetError::ValueTooLarge);
            }
            if self.remaining() < len as usize {
                return Err(TryGetError::NotEnoughBytes);
            }

            let mut result = String::with_capacity(len as usize);
            self.take(len as u64)
                .read_to_string(&mut result)
                .map_err(|_| TryGetError::NotEnoughBytes)?;

            return Ok(result);
        }

        Err(TryGetError::NotEnoughBytes)
    }

    fn try_get_position(&mut self) -> Result<BlockPosition, TryGetError> {
        let val = self.try_get_i64()?;
        let x = val >> 38;
        let y = (val >> 26) & 0xFFF;
        let z = val << 38 >> 38;

        Ok(BlockPosition::new(x as i32, y as i32, z as i32))
    }

    fn try_get_bool(&mut self) -> Result<bool, TryGetError> {
        let byte = self.try_get_i8()?;
        match byte {
            0 => Ok(false),
            1 => Ok(true),
            x => Err(TryGetError::InvalidValue(i32::from(x))),
        }
    }

    fn try_get_uuid(&mut self) -> anyhow::Result<Uuid> {
        let mut bytes = [0u8; 16];
        self.read_exact(&mut bytes)?;
        Ok(Uuid::from_bytes(bytes))
    }

    fn try_get_nbt<D: DeserializeOwned>(&mut self) -> Result<D, nbt::Error> {
        nbt::from_reader(self)
    }

    fn try_get_slot(&mut self) -> Result<Option<ItemStack>, TryGetError> {
        let present = self.try_get_bool()?;

        if !present {
            return Ok(None);
        }

        let id = self.try_get_var_int()?;
        let ty = Item::from_vanilla_id(id as u32).ok_or(TryGetError::InvalidValue(id))?;
        let amount = self.try_get_i8()? as u8;
        let nbt: Option<ItemNbt> = self.try_get_nbt().ok();

        Ok(Some(ItemStack {
            ty,
            amount,
            damage: nbt.map(|t| t.damage).flatten(),
        }))
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
        for (index, entry) in meta.iter() {
            self.push_u8(index);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    macro_rules! test_var_int_rw {
        ($([$($byte:literal),* $(,)?] => $result:literal)*) => {
            $(
                assert_eq!(
                    Cursor::new(bytes::Bytes::from(vec![$($byte),*])).try_get_var_int().unwrap(),
                    $result
                );
                let mut bytes = BytesMut::new();
                bytes.push_var_int($result);
                assert_eq!(bytes.as_ref(), [$($byte),*]);
            )*
        };
    }

    #[test]
    fn test_var_int() {
        test_var_int_rw! {
            [0] => 0
            [1] => 1
            [2] => 2
            [127] => 127
            [128, 1] => 128
            [255, 1] => 255
            [255, 255, 127] => 2097151
            [255, 255, 255, 255, 7] => 2147483647
            [255, 255, 255, 255, 15] => -1
            [128, 128, 128, 128, 8] => -2147483648
        }
    }
}
