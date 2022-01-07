//! Traits for reading/writing Minecraft-encoded values.

use crate::{ProtocolVersion, Slot};
use anyhow::{anyhow, bail, Context};
use base::{
    anvil::entity::ItemNbt, metadata::MetaEntry, BlockId, BlockPosition, Direction, EntityMetadata,
    Gamemode, Item, ItemStackBuilder, ValidBlockPosition,
};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use libcraft_items::InventorySlot::*;
use num_traits::{FromPrimitive, ToPrimitive};
use quill_common::components::PreviousGamemode;
use serde::{de::DeserializeOwned, Serialize};
use std::io::ErrorKind;
use std::{
    borrow::Cow,
    collections::BTreeMap,
    convert::{TryFrom, TryInto},
    io::{self, Cursor, Read, Write},
    iter,
    marker::PhantomData,
    num::TryFromIntError,
};
use thiserror::Error;
use uuid::Uuid;

/// Trait implemented for types which can be read
/// from a buffer.
pub trait Readable {
    /// Reads this type from the given buffer.
    fn read(buffer: &mut Cursor<&[u8]>, version: ProtocolVersion) -> anyhow::Result<Self>
    where
        Self: Sized;
}

/// Trait implemented for types which can be written
/// to a buffer.
pub trait Writeable: Sized {
    /// Writes this value to the given buffer.
    fn write(&self, buffer: &mut Vec<u8>, version: ProtocolVersion) -> anyhow::Result<()>;
}

impl<'a, T> Writeable for &'a T
where
    T: Writeable,
{
    fn write(&self, buffer: &mut Vec<u8>, version: ProtocolVersion) -> anyhow::Result<()> {
        T::write(*self, buffer, version)?;
        Ok(())
    }
}

/// Error when reading a value.
#[derive(Debug, Error)]
pub enum Error {
    #[error("unexpected end of input: failed to read value of type `{0}`")]
    UnexpectedEof(&'static str),
}

macro_rules! integer_impl {
    ($($int:ty, $read_fn:tt, $write_fn:tt),* $(,)?) => {
        $(
            impl Readable for $int {
                fn read(buffer: &mut Cursor<&[u8]>, _version: ProtocolVersion) -> anyhow::Result<Self> {
                    buffer.$read_fn::<BigEndian>().map_err(anyhow::Error::from)
                }
            }

            impl Writeable for $int {
                fn write(&self, buffer: &mut Vec<u8>, _version: ProtocolVersion) -> anyhow::Result<()> {
                    buffer.$write_fn::<BigEndian>(*self)?;
                    Ok(())
                }
            }
        )*
    }
}

integer_impl! {
    u16, read_u16, write_u16,
    u32, read_u32, write_u32,
    u64, read_u64, write_u64,

    i16, read_i16, write_i16,
    i32, read_i32, write_i32,
    i64, read_i64, write_i64,

    f32, read_f32, write_f32,
    f64, read_f64, write_f64,
}

impl Readable for u8 {
    fn read(buffer: &mut Cursor<&[u8]>, _version: ProtocolVersion) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        buffer.read_u8().map_err(anyhow::Error::from)
    }
}

impl Writeable for u8 {
    fn write(&self, buffer: &mut Vec<u8>, _version: ProtocolVersion) -> anyhow::Result<()> {
        buffer.write_u8(*self)?;
        Ok(())
    }
}

impl Readable for i8 {
    fn read(buffer: &mut Cursor<&[u8]>, _version: ProtocolVersion) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        buffer.read_i8().map_err(anyhow::Error::from)
    }
}

impl Writeable for i8 {
    fn write(&self, buffer: &mut Vec<u8>, _version: ProtocolVersion) -> anyhow::Result<()> {
        buffer.write_i8(*self)?;
        Ok(())
    }
}

impl<T> Readable for Option<T>
where
    T: Readable,
{
    fn read(buffer: &mut Cursor<&[u8]>, version: ProtocolVersion) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        // Assume boolean prefix.
        let present = bool::read(buffer, version)?;

        if present {
            Ok(Some(T::read(buffer, version)?))
        } else {
            Ok(None)
        }
    }
}

impl<T> Writeable for Option<T>
where
    T: Writeable,
{
    fn write(&self, buffer: &mut Vec<u8>, version: ProtocolVersion) -> anyhow::Result<()> {
        let present = self.is_some();
        present.write(buffer, version)?;

        if let Some(value) = self {
            value.write(buffer, version)?;
        }

        Ok(())
    }
}

/// A variable-length integer as defined by the Minecraft protocol.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VarInt(pub i32);

impl Readable for VarInt {
    fn read(buffer: &mut Cursor<&[u8]>, _version: ProtocolVersion) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Self::read_from(buffer).map_err(Into::into)
    }
}

impl TryFrom<VarInt> for usize {
    type Error = TryFromIntError;
    fn try_from(value: VarInt) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl From<usize> for VarInt {
    fn from(x: usize) -> Self {
        VarInt(x as i32)
    }
}

impl From<VarInt> for i32 {
    fn from(x: VarInt) -> Self {
        x.0
    }
}

impl From<i32> for VarInt {
    fn from(x: i32) -> Self {
        VarInt(x)
    }
}

impl VarInt {
    pub fn write_to(&self, mut writer: impl Write) -> io::Result<usize> {
        let mut x = self.0 as u32;
        let mut i = 0;
        loop {
            let mut temp = (x & 0b0111_1111) as u8;
            x >>= 7;
            if x != 0 {
                temp |= 0b1000_0000;
            }

            writer.write_all(&[temp])?;

            i += 1;
            if x == 0 {
                break;
            }
        }
        Ok(i)
    }
    pub fn read_from(mut reader: impl Read) -> io::Result<Self> {
        let mut num_read = 0;
        let mut result = 0;

        loop {
            let read = reader.read_u8()?;
            let value = i32::from(read & 0b0111_1111);
            result |= value.overflowing_shl(7 * num_read).0;

            num_read += 1;

            if num_read > 5 {
                return Err(io::Error::new(
                    ErrorKind::InvalidData,
                    "VarInt too long (max length: 5)",
                ));
            }
            if read & 0b1000_0000 == 0 {
                break;
            }
        }
        Ok(VarInt(result))
    }
}

impl Writeable for VarInt {
    fn write(&self, buffer: &mut Vec<u8>, _version: ProtocolVersion) -> anyhow::Result<()> {
        self.write_to(buffer).expect("write to Vec failed");
        Ok(())
    }
}

/// A variable-length integer as defined by the Minecraft protocol.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VarLong(pub i64);

impl Readable for VarLong {
    fn read(buffer: &mut Cursor<&[u8]>, version: ProtocolVersion) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let mut num_read = 0;
        let mut result = 0;

        loop {
            let read = u8::read(buffer, version)?;
            let value = i64::from(read & 0b0111_1111);
            result |= value.overflowing_shl(7 * num_read).0;

            num_read += 1;

            if num_read > 10 {
                bail!(
                    "VarInt too long (max length: 5, value read so far: {})",
                    result
                );
            }
            if read & 0b1000_0000 == 0 {
                break;
            }
        }
        Ok(VarLong(result))
    }
}

impl From<VarLong> for i64 {
    fn from(x: VarLong) -> Self {
        x.0
    }
}

impl From<i64> for VarLong {
    fn from(x: i64) -> Self {
        VarLong(x)
    }
}

impl Writeable for VarLong {
    fn write(&self, buffer: &mut Vec<u8>, _version: ProtocolVersion) -> anyhow::Result<()> {
        let mut x = self.0 as u64;
        loop {
            let mut temp = (x & 0b0111_1111) as u8;
            x >>= 7;
            if x != 0 {
                temp |= 0b1000_0000;
            }

            buffer.write_u8(temp).unwrap();

            if x == 0 {
                break;
            }
        }

        Ok(())
    }
}

impl Readable for String {
    fn read(buffer: &mut Cursor<&[u8]>, version: ProtocolVersion) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        // Length is encoded as VarInt.
        // Following `length` bytes are the UTF8-encoded
        // string.

        let length = VarInt::read(buffer, version)
            .context("failed to read string length")?
            .0 as usize;

        // TODO: support custom length limits
        // Current max length is max value of a signed 16-bit int.
        let max_length = std::i16::MAX as usize;
        if length > max_length {
            bail!(
                "string length {} exceeds maximum allowed length of {}",
                length,
                max_length
            );
        }

        // Read string into buffer.
        let mut temp = vec![0u8; length];
        buffer
            .read_exact(&mut temp)
            .map_err(|_| Error::UnexpectedEof("String"))?;
        let s = std::str::from_utf8(&temp).context("string contained invalid UTF8")?;

        Ok(s.to_owned())
    }
}

impl Writeable for String {
    fn write(&self, buffer: &mut Vec<u8>, version: ProtocolVersion) -> anyhow::Result<()> {
        VarInt(self.len() as i32).write(buffer, version)?;
        buffer.extend_from_slice(self.as_bytes());

        Ok(())
    }
}

impl Readable for bool {
    fn read(buffer: &mut Cursor<&[u8]>, version: ProtocolVersion) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let x = u8::read(buffer, version)?;

        if x == 0 {
            Ok(false)
        } else if x == 1 {
            Ok(true)
        } else {
            Err(anyhow::anyhow!("invalid boolean tag {}", x))
        }
    }
}

impl Writeable for bool {
    fn write(&self, buffer: &mut Vec<u8>, version: ProtocolVersion) -> anyhow::Result<()> {
        let x = if *self { 1u8 } else { 0 };
        x.write(buffer, version)?;

        Ok(())
    }
}

pub const MAX_LENGTH: usize = 1024 * 1024; // 2^20 elements

/// Reads and writes an array of inner `Writeable`s.
/// The array is prefixed with a `VarInt` length.
///
/// This will reject arrays of lengths larger than MAX_LENGTH.
pub struct LengthPrefixedVec<'a, P, T>(pub Cow<'a, [T]>, PhantomData<P>)
where
    [T]: ToOwned<Owned = Vec<T>>;

impl<'a, P, T> Readable for LengthPrefixedVec<'a, P, T>
where
    T: Readable,
    [T]: ToOwned<Owned = Vec<T>>,
    P: TryInto<usize> + Readable,
    P::Error: std::error::Error + Send + Sync + 'static,
{
    fn read(buffer: &mut Cursor<&[u8]>, version: ProtocolVersion) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let length: usize = P::read(buffer, version)?.try_into()?;

        if length > MAX_LENGTH {
            bail!("array length too large ({} > {})", length, MAX_LENGTH);
        }

        let vec = iter::repeat_with(|| T::read(buffer, version))
            .take(length)
            .collect::<anyhow::Result<Vec<T>>>()?;
        Ok(Self(Cow::Owned(vec), PhantomData))
    }
}

impl<'a, P, T> Writeable for LengthPrefixedVec<'a, P, T>
where
    T: Writeable,
    [T]: ToOwned<Owned = Vec<T>>,
    P: TryFrom<usize> + Writeable,
    P::Error: std::error::Error + Send + Sync + 'static,
{
    fn write(&self, buffer: &mut Vec<u8>, version: ProtocolVersion) -> anyhow::Result<()> {
        P::try_from(self.0.len())?.write(buffer, version)?;
        self.0
            .iter()
            .for_each(|item| item.write(buffer, version).expect("failed to write to vec"));

        Ok(())
    }
}

impl<'a, P, T> From<LengthPrefixedVec<'a, P, T>> for Vec<T>
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    fn from(x: LengthPrefixedVec<'a, P, T>) -> Self {
        x.0.into_owned()
    }
}

impl<'a, P, T> From<&'a [T]> for LengthPrefixedVec<'a, P, T>
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    fn from(slice: &'a [T]) -> Self {
        Self(Cow::Borrowed(slice), PhantomData)
    }
}

impl<'a, P, T> From<Vec<T>> for LengthPrefixedVec<'a, P, T>
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    fn from(vec: Vec<T>) -> Self {
        Self(Cow::Owned(vec), PhantomData)
    }
}

pub type VarIntPrefixedVec<'a, T> = LengthPrefixedVec<'a, VarInt, T>;
pub type ShortPrefixedVec<'a, T> = LengthPrefixedVec<'a, u16, T>;

/// A vector of bytes which consumes all remaining bytes in this packet.
/// This is used by the plugin messaging packets, for one.
pub struct LengthInferredVecU8<'a>(pub Cow<'a, [u8]>);

impl<'a> Readable for LengthInferredVecU8<'a> {
    fn read(buffer: &mut Cursor<&[u8]>, _version: ProtocolVersion) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let mut vec = Vec::new();
        buffer.read_to_end(&mut vec)?;
        Ok(LengthInferredVecU8(Cow::Owned(vec)))
    }
}

impl<'a> Writeable for LengthInferredVecU8<'a> {
    fn write(&self, buffer: &mut Vec<u8>, _version: ProtocolVersion) -> anyhow::Result<()> {
        buffer.extend_from_slice(&*self.0);
        Ok(())
    }
}

impl<'a> From<&'a [u8]> for LengthInferredVecU8<'a> {
    fn from(slice: &'a [u8]) -> Self {
        LengthInferredVecU8(Cow::Borrowed(slice))
    }
}

impl<'a> From<LengthInferredVecU8<'a>> for Vec<u8> {
    fn from(x: LengthInferredVecU8<'a>) -> Self {
        x.0.into_owned()
    }
}

/// Wrapper over an arbitrary type that implements `Deserialize` and `Serialize`.
///
/// The value will be written to a packet as NBT data.
#[derive(Debug, Clone)]
pub struct Nbt<T>(pub T);

impl<T> Readable for Nbt<T>
where
    T: DeserializeOwned,
{
    fn read(buffer: &mut Cursor<&[u8]>, _version: ProtocolVersion) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        nbt::from_reader(buffer)
            .map_err(anyhow::Error::from)
            .map(Nbt)
    }
}

impl<T> Writeable for Nbt<T>
where
    T: Serialize,
{
    fn write(&self, buffer: &mut Vec<u8>, _version: ProtocolVersion) -> anyhow::Result<()> {
        nbt::to_writer(buffer, &self.0, None).unwrap_or_else(|e| {
            panic!(
                "could not serialize struct of type '{}' to NBT: {}",
                std::any::type_name::<T>(),
                e
            )
        });

        Ok(())
    }
}

impl<T> From<T> for Nbt<T> {
    fn from(t: T) -> Self {
        Nbt(t)
    }
}

impl Readable for Slot {
    fn read(buffer: &mut Cursor<&[u8]>, version: ProtocolVersion) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let present = bool::read(buffer, version)?;

        if present {
            let item_id = VarInt::read(buffer, version)?.0;
            let count = u8::read(buffer, version)? as u32;

            // Read NBT, but make sure to reset the buffer position if it's missing.
            let position = buffer.position();
            let tags: Option<ItemNbt> = Nbt::read(buffer, version).ok().map(|nbt| nbt.0);
            if tags.is_none() {
                buffer.set_position(position + 1); // account for TAG_End, which is 1 byte
            }

            let item = Item::from_id(item_id.try_into()?)
                .ok_or_else(|| anyhow!("unknown item ID {}", item_id))?;

            // Todo fix: Panics if count is zero
            Ok(Filled(
                ItemStackBuilder::with_item(item)
                    .count(count)
                    .apply_damage(tags.map(|t| t.damage).flatten())
                    .into(),
            ))
        } else {
            Ok(Empty)
        }
    }
}

impl Writeable for Slot {
    fn write(&self, buffer: &mut Vec<u8>, version: ProtocolVersion) -> anyhow::Result<()> {
        self.is_filled().write(buffer, version)?;

        if let Filled(stack) = self {
            VarInt(stack.item().id() as i32).write(buffer, version)?;
            (stack.count() as u8).write(buffer, version)?;

            let tags: ItemNbt = stack.into();
            if tags != ItemNbt::default() {
                Nbt(tags).write(buffer, version)?;
            } else {
                0u8.write(buffer, version)?; // TAG_End
            }
        }

        Ok(())
    }
}

impl Readable for EntityMetadata {
    fn read(buffer: &mut Cursor<&[u8]>, version: ProtocolVersion) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let mut values = BTreeMap::new();

        loop {
            let index = u8::read(buffer, version)?;

            if index == 0xFF {
                break;
            }

            let entry = read_meta_entry(buffer, version)?;
            values.insert(index, entry);
        }

        Ok(EntityMetadata { values })
    }
}

fn read_meta_entry(
    buffer: &mut Cursor<&[u8]>,
    version: ProtocolVersion,
) -> anyhow::Result<MetaEntry> {
    let id = VarInt::read(buffer, version)?.0;

    Ok(match id {
        0 => MetaEntry::Byte(i8::read(buffer, version)?),
        1 => MetaEntry::VarInt(VarInt::read(buffer, version)?.0),
        2 => MetaEntry::Float(f32::read(buffer, version)?),
        3 => MetaEntry::String(String::read(buffer, version)?),
        4 => MetaEntry::Chat(String::read(buffer, version)?),
        5 => MetaEntry::OptChat(if bool::read(buffer, version)? {
            Some(String::read(buffer, version)?)
        } else {
            None
        }),
        6 => MetaEntry::Slot(Slot::read(buffer, version)?),
        7 => MetaEntry::Boolean(bool::read(buffer, version)?),
        8 => MetaEntry::Rotation(
            f32::read(buffer, version)?,
            f32::read(buffer, version)?,
            f32::read(buffer, version)?,
        ),
        9 => MetaEntry::Position(ValidBlockPosition::read(buffer, version)?),
        10 => MetaEntry::OptPosition(if bool::read(buffer, version)? {
            Some(ValidBlockPosition::read(buffer, version)?)
        } else {
            None
        }),
        11 => MetaEntry::Direction(
            Direction::from_i32(VarInt::read(buffer, version)?.0)
                .ok_or_else(|| anyhow!("invalid direction ID"))?,
        ),
        12 => MetaEntry::OptUuid(if bool::read(buffer, version)? {
            Some(Uuid::read(buffer, version)?)
        } else {
            None
        }),
        13 => MetaEntry::OptBlockId({
            let id = VarInt::read(buffer, version)?.0;
            if id == 0 {
                None
            } else {
                Some(id)
            }
        }),
        14 => MetaEntry::Nbt(Nbt::read(buffer, version)?.0),
        15 => MetaEntry::Particle,
        16 => MetaEntry::VillagerData(
            VarInt::read(buffer, version)?.0,
            VarInt::read(buffer, version)?.0,
            VarInt::read(buffer, version)?.0,
        ),
        17 => MetaEntry::OptVarInt({
            let varint = VarInt::read(buffer, version)?.0;
            if varint == 0 {
                None
            } else {
                Some(varint - 1)
            }
        }),
        18 => MetaEntry::Pose(VarInt::read(buffer, version)?.0),
        x => bail!("invalid entity metadata entry ID {}", x),
    })
}

impl Writeable for EntityMetadata {
    fn write(&self, buffer: &mut Vec<u8>, version: ProtocolVersion) -> anyhow::Result<()> {
        for (index, entry) in self.iter() {
            index.write(buffer, version)?;
            VarInt(entry.id()).write(buffer, version)?;
            write_meta_entry(entry, buffer, version)?;
        }

        // End of metadata
        buffer.push(0xFF);
        Ok(())
    }
}

fn write_meta_entry(
    entry: &MetaEntry,
    buffer: &mut Vec<u8>,
    version: ProtocolVersion,
) -> anyhow::Result<()> {
    match entry {
        MetaEntry::Byte(x) => x.write(buffer, version)?,
        MetaEntry::VarInt(x) => {
            VarInt(*x).write(buffer, version)?;
        }
        MetaEntry::Float(x) => x.write(buffer, version)?,
        MetaEntry::String(x) => x.write(buffer, version)?,
        MetaEntry::Chat(x) => x.write(buffer, version)?,
        MetaEntry::OptChat(ox) => {
            if let Some(x) = ox {
                true.write(buffer, version)?;
                x.write(buffer, version)?;
            } else {
                false.write(buffer, version)?;
            }
        }
        MetaEntry::Slot(slot) => slot.write(buffer, version)?,
        MetaEntry::Boolean(x) => x.write(buffer, version)?,
        MetaEntry::Rotation(x, y, z) => {
            x.write(buffer, version)?;
            y.write(buffer, version)?;
            z.write(buffer, version)?;
        }
        MetaEntry::Position(x) => x.write(buffer, version)?,
        MetaEntry::OptPosition(ox) => {
            if let Some(x) = ox {
                true.write(buffer, version)?;
                x.write(buffer, version)?;
            } else {
                false.write(buffer, version)?;
            }
        }
        MetaEntry::Direction(x) => VarInt(x.to_i32().unwrap()).write(buffer, version)?,
        MetaEntry::OptUuid(ox) => {
            if let Some(x) = ox {
                true.write(buffer, version)?;
                x.write(buffer, version)?;
            } else {
                false.write(buffer, version)?;
            }
        }
        MetaEntry::OptBlockId(ox) => {
            if let Some(x) = ox {
                VarInt(*x).write(buffer, version)?;
            } else {
                VarInt(0).write(buffer, version)?; // No value implies air
            }
        }
        MetaEntry::Nbt(val) => Nbt(val).write(buffer, version)?,
        MetaEntry::Particle => unimplemented!("entity metadata with particles"),
        MetaEntry::VillagerData(villager_type, villager_profession, level) => {
            VarInt(*villager_type).write(buffer, version)?;
            VarInt(*villager_profession).write(buffer, version)?;
            VarInt(*level).write(buffer, version)?;
        }
        MetaEntry::OptVarInt(ox) => {
            if let Some(x) = ox {
                true.write(buffer, version)?;
                x.write(buffer, version)?;
            } else {
                false.write(buffer, version)?;
            }
        }
        MetaEntry::Pose(x) => VarInt(x.to_i32().unwrap()).write(buffer, version)?,
    }

    Ok(())
}

impl Readable for Uuid {
    fn read(buffer: &mut Cursor<&[u8]>, _version: ProtocolVersion) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let mut bytes = uuid::Bytes::default();
        buffer.read_exact(&mut bytes)?;

        Ok(Uuid::from_bytes(bytes))
    }
}

impl Writeable for Uuid {
    fn write(&self, buffer: &mut Vec<u8>, _version: ProtocolVersion) -> anyhow::Result<()> {
        buffer.extend_from_slice(self.as_bytes());
        Ok(())
    }
}

impl Readable for ValidBlockPosition {
    fn read(buffer: &mut Cursor<&[u8]>, version: ProtocolVersion) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let val = i64::read(buffer, version)?;

        let x = (val >> 38) as i32;
        let y = (val & 0xFFF) as i32;
        let z = (val << 26 >> 38) as i32;

        Ok(BlockPosition { x, y, z }.try_into()?)
    }
}

impl Writeable for ValidBlockPosition {
    fn write(&self, buffer: &mut Vec<u8>, version: ProtocolVersion) -> anyhow::Result<()> {
        let val = ((self.x() as u64 & 0x3FFFFFF) << 38)
            | ((self.z() as u64 & 0x3FFFFFF) << 12)
            | (self.y() as u64 & 0xFFF);
        val.write(buffer, version)?;

        Ok(())
    }
}

/// An angle written in stops, where each stop
/// is 1/256th of a full turn.
///
/// This type converts degrees to stops.
#[derive(Copy, Clone, Debug)]
pub struct Angle(pub f32);

impl From<Angle> for f32 {
    fn from(angle: Angle) -> Self {
        angle.0
    }
}

impl Readable for Angle {
    fn read(buffer: &mut Cursor<&[u8]>, version: ProtocolVersion) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let val = u8::read(buffer, version)?;
        Ok(Angle((val as f32 / 256.0) * 360.0))
    }
}

impl Writeable for Angle {
    fn write(&self, buffer: &mut Vec<u8>, version: ProtocolVersion) -> anyhow::Result<()> {
        let temp = (256.0 / 360.0) * (self.0 % 360.0);
        // Wrap negative values 'x' in the range [-256.0 to 0] to the
        // correct angle in the range [0 to 256.0 ) by changing 'x' to
        // x = 256.0 - x
        let val = ((temp + 256.0) % 256.0) as u8;
        val.write(buffer, version)?;

        Ok(())
    }
}

impl Readable for BlockId {
    fn read(buffer: &mut Cursor<&[u8]>, version: ProtocolVersion) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let id = VarInt::read(buffer, version)?.0;

        let block = BlockId::from_vanilla_id(id.try_into()?);
        Ok(block)
    }
}

impl Writeable for BlockId {
    fn write(&self, buffer: &mut Vec<u8>, version: ProtocolVersion) -> anyhow::Result<()> {
        VarInt(self.vanilla_id().into()).write(buffer, version)?;
        Ok(())
    }
}

impl Readable for Gamemode {
    fn read(buffer: &mut Cursor<&[u8]>, version: ProtocolVersion) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let id = u8::read(buffer, version)?;
        Ok(match id {
            0 => Gamemode::Survival,
            1 => Gamemode::Creative,
            2 => Gamemode::Adventure,
            3 => Gamemode::Spectator,
            id => bail!("invalid gamemode ID {}", id),
        })
    }
}

impl Writeable for Gamemode {
    fn write(&self, buffer: &mut Vec<u8>, version: ProtocolVersion) -> anyhow::Result<()> {
        let id = match self {
            Gamemode::Survival => 0,
            Gamemode::Creative => 1,
            Gamemode::Adventure => 2,
            Gamemode::Spectator => 3,
        };
        (id as u8).write(buffer, version)?;

        Ok(())
    }
}

impl Readable for PreviousGamemode {
    fn read(buffer: &mut Cursor<&[u8]>, version: ProtocolVersion) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self::from_id(i8::read(buffer, version)?))
    }
}

impl Writeable for PreviousGamemode {
    fn write(&self, buffer: &mut Vec<u8>, version: ProtocolVersion) -> anyhow::Result<()> {
        self.id().write(buffer, version)
    }
}
