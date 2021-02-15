use libcraft_blocks_data::RawBlockStateProperties;

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
#[derive(Debug)]
pub struct Ageable {
    pub age: u8,
}

impl BlockData for Ageable {
    fn from_raw(raw: &RawBlockStateProperties) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Self { age: raw.age? })
    }

    fn apply(&self, raw: &mut RawBlockStateProperties) {
        raw.age.replace(self.age);
    }
}

/// A block that can be powered with a redstone
/// signal.
#[derive(Debug)]
pub struct AnaloguePowerable {
    /// Typically from 0 to 15, inclusive.
    pub power: u8,
}

impl BlockData for AnaloguePowerable {
    fn from_raw(raw: &RawBlockStateProperties) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Self { power: raw.power? })
    }

    fn apply(&self, raw: &mut RawBlockStateProperties) {
        raw.age.replace(self.power);
    }
}

// TODO: implement all `BlockData`s displayed at
// https://hub.spigotmc.org/javadocs/spigot/org/bukkit/block/data/BlockData.html
