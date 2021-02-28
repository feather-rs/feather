use crate::data::RawBlockStateProperties;
use libcraft_macros::BlockData;

/// Represents the data (properties) of a block.
///
/// Types implementing this trait mirror Bukkit's `BlockData` interface.
///
/// This trait is internal; don't try implementing it for your
/// own types.
pub trait BlockData {
    fn from_raw(raw: &RawBlockStateProperties) -> Option<Self>
    where
        Self: Sized;

    fn apply(&self, raw: &mut RawBlockStateProperties);
}

/// A block that has an "age" property that
/// represents crop growth.
///
/// Fire also has this property.
#[derive(Debug, BlockData)]
pub struct Ageable {
    pub age: u8,
}

/// A block that can be powered with a redstone
/// signal.
#[derive(Debug, BlockData)]
pub struct AnaloguePowerable {
    /// Typically from 0 to 15, inclusive.
    pub power: u8,
}

// TODO: implement all `BlockData`s displayed at
// https://hub.spigotmc.org/javadocs/spigot/org/bukkit/block/data/BlockData.html
