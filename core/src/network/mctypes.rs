use crate::bytebuf::{BufMutAlloc, BufResulted, ByteBuf};
use crate::inventory::ItemStack;
use crate::prelude::*;
use crate::world::BlockPosition;
use bytes::Buf;
use feather_items::{Item, ItemExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

/// Identifies a type to which Minecraft-specific
/// types (`VarInt`, `VarLong`, etc.) can be written.
pub trait McTypeWrite {
    /// Writes a `VarInt` to the object. See wiki.vg for
    /// details on `VarInt`s and related types.
    fn write_var_int(&mut self, x: i32);
    /// Writes a string to the object. This method
    /// will first write the length of the string in bytes
    /// encodes as a `VarInt` and will then write
    /// the UTF-8 bytes of the string.
    fn write_string(&mut self, x: &str);

    fn write_position(&mut self, x: &BlockPosition);

    fn write_bool(&mut self, x: bool);

    fn write_uuid(&mut self, x: &Uuid);

    fn write_nbt<T: Serialize>(&mut self, x: &T);

    fn write_slot(&mut self, slot: &Option<ItemStack>);
}

/// Identifies a type from which Minecraft-specified
/// types can be read.
pub trait McTypeRead {
    /// Reads a `VarInt` from this object, returning
    /// `Some(x)` if successful or `None` if the object
    /// does not contain a valid `VarInt`.
    fn read_var_int(&mut self) -> Result<i32, ()>;
    /// Reads a string from the object.
    fn read_string(&mut self) -> Result<String, ()>;

    fn read_position(&mut self) -> Result<BlockPosition, ()>;

    fn read_bool(&mut self) -> Result<bool, ()>;

    fn read_uuid(&mut self) -> Result<Uuid, ()>;

    fn read_nbt<'de, T: Deserialize<'de>>(&mut self) -> Result<T, nbt::Error>;

    fn read_slot(&mut self) -> Result<Option<ItemStack>, ()>;
}

impl McTypeWrite for ByteBuf {
    /// Writes a `VarInt` to the object. See wiki.vg for
    /// details on `VarInt`s and related types.
    fn write_var_int(&mut self, mut x: i32) {
        loop {
            let mut temp = (x & 0b0111_1111) as u8;
            x >>= 7;
            if x != 0 {
                temp |= 0b1000_0000;
            }
            self.write_u8(temp);
            if x == 0 {
                break;
            }
        }
    }

    /// Writes a string to the object. This method
    /// will first write the length of the string in bytes
    /// encodes as a `VarInt` and will then write
    /// the UTF-8 bytes of the string.
    fn write_string(&mut self, x: &str) {
        let len = x.len();
        self.write_var_int(len as i32);

        let bytes = x.as_bytes();
        self.write_all(bytes).unwrap();
    }

    fn write_position(&mut self, x: &BlockPosition) {
        let result: u64 = ((x.x as u64 & 0x03FF_FFFF) << 38)
            | ((x.y as u64 & 0xFFF) << 26)
            | (x.z as u64 & 0x03FF_FFFF);

        self.write_u64_be(result);
    }

    fn write_bool(&mut self, x: bool) {
        if x {
            self.write_u8(1);
        } else {
            self.write_u8(0);
        }
    }

    fn write_uuid(&mut self, x: &Uuid) {
        self.write_all(&x.as_bytes()[..]).unwrap();
    }

    fn write_nbt<T: Serialize>(&mut self, val: &T) {
        nbt::to_writer(self, val, None).unwrap(); // Unwrap is safe because writing would only fail if a struct couldn't be written
    }

    fn write_slot(&mut self, slot: &Option<ItemStack>) {
        self.write_bool(slot.is_some());

        if let Some(slot) = slot.as_ref() {
            self.write_var_int(slot.ty.native_protocol_id());
            self.write_i8(slot.amount as i8);
            self.write_u8(0x00); // TAG_End - TODO item NBT support
        }
    }
}

impl<T: Buf + Read> McTypeRead for T {
    /// Reads a `VarInt` from this object, returning
    /// `Some(x)` if successful or `None` if the object
    /// does not contain a valid `VarInt`.
    fn read_var_int(&mut self) -> Result<i32, ()> {
        let mut num_read = 0;
        let mut result = 0;
        loop {
            if self.remaining() == 0 {
                return Err(());
            }
            let read = self.read_u8()?;
            let value = read & 0b0111_1111;
            result |= (i32::from(value)) << (7 * num_read);

            num_read += 1;
            if num_read > 5 {
                return Err(());
            }
            if read & 0b1000_0000 == 0 {
                break;
            }
        }
        Ok(result)
    }

    /// Reads a string from the object.
    fn read_string(&mut self) -> Result<String, ()> {
        let len = self.read_var_int();
        if let Ok(len) = len {
            // Check that the client isn't trying
            // to make the server allocate ridiculous
            // amounts of memory
            if len > 32767 {
                return Err(());
            }
            let mut result = String::with_capacity(len as usize);
            for _ in 0..len {
                result.push(self.get_u8() as char);
            }
            return Ok(result);
        }

        Err(())
    }

    fn read_position(&mut self) -> Result<BlockPosition, ()> {
        if self.remaining() < 8 {
            return Err(());
        }
        let val = self.read_i64_be()?;
        let x = val >> 38;
        let y = (val >> 26) & 0xFFF;
        let z = val << 38 >> 38;

        Ok(BlockPosition::new(x as i32, y as i32, z as i32))
    }

    fn read_bool(&mut self) -> Result<bool, ()> {
        let byte = self.read_u8()?;
        match byte {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(()),
        }
    }

    fn read_uuid(&mut self) -> Result<Uuid, ()> {
        let mut bytes = [0; 16];
        self.read(&mut bytes).map_err(|_| ())?;
        Ok(Uuid::from_bytes(bytes))
    }

    fn read_nbt<'de, D: Deserialize<'de>>(&mut self) -> Result<D, nbt::Error> {
        unimplemented!()
    }

    fn read_slot(&mut self) -> Result<Option<ItemStack>, ()> {
        let present = self.read_bool()?;

        if !present {
            return Ok(None);
        }

        let id = self.read_var_int()?;
        let ty = Item::from_native_protocol_id(id).ok_or(())?;
        let amount = self.read_u8()?;

        // TODO NBT support
        warn!("NBT unsupported; client will probably be kicked");

        Ok(Some(ItemStack::new(ty, amount)))
    }
}
