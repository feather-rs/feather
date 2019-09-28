use crate::bytes_ext::{BytesExt, BytesMutExt, TryGetError};
use crate::inventory::ItemStack;
use crate::prelude::*;
use crate::world::BlockPosition;
use bytes::{Buf, BufMut, BytesMut};
use feather_items::{Item, ItemExt};
use serde::{Deserialize, Serialize};
use std::io::Read;

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

    fn push_slot(&mut self, slot: &Option<ItemStack>);
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

    fn try_get_uuid(&mut self) -> Result<Uuid, TryGetError>;

    fn try_get_nbt<'de, T: Deserialize<'de>>(&mut self) -> Result<T, nbt::Error>;

    fn try_get_slot(&mut self) -> Result<Option<ItemStack>, TryGetError>;
}

impl McTypeWrite for BytesMut {
    fn push_var_int(&mut self, mut x: i32) -> usize {
        let mut bytes_written = 0;
        loop {
            let mut temp = (x & 0b0111_1111) as u8;
            x >>= 7;
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
        let len = x.len();
        self.push_var_int(len as i32);

        let bytes = x.as_bytes();
        self.reserve(bytes.len());
        self.put(bytes);
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

    fn push_slot(&mut self, slot: &Option<ItemStack>) {
        self.push_bool(slot.is_some());

        if let Some(slot) = slot.as_ref() {
            self.push_var_int(slot.ty.native_protocol_id());
            self.push_i8(slot.amount as i8);
            self.push_i8(0x00); // TAG_End - TODO item NBT support
        }
    }
}

impl<B: Buf> McTypeRead for B {
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
            result |= value << (7 * num_read);

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
            let mut result = String::with_capacity(len as usize);
            for _ in 0..len {
                let res = self.try_get_i8()? as u8;
                result.push(res as char);
            }

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
            _ => Err(TryGetError::InvalidValue),
        }
    }

    fn try_get_uuid(&mut self) -> Result<Uuid, TryGetError> {
        let mut bytes = [0u8; 16];
        self.bytes()
            .read(&mut bytes)
            .map_err(|_| TryGetError::NotEnoughBytes)?;
        Ok(Uuid::from_bytes(bytes))
    }

    fn try_get_nbt<'de, D: Deserialize<'de>>(&mut self) -> Result<D, nbt::Error> {
        unimplemented!()
    }

    fn try_get_slot(&mut self) -> Result<Option<ItemStack>, TryGetError> {
        let present = self.try_get_bool()?;

        if !present {
            return Ok(None);
        }

        let id = self.try_get_var_int()?;
        let ty = Item::from_native_protocol_id(id).ok_or(TryGetError::InvalidValue)?;
        let amount = self.try_get_i8()? as u8;

        // TODO NBT support

        Ok(Some(ItemStack::new(ty, amount)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_var_int() {
        // Examples from wiki.vg
        let mut buf = ByteBuf::new();
        buf.write_all(&[0xff, 0x01]).unwrap();
        assert_eq!(buf.try_get_var_int(), Ok(255));
    }
}
