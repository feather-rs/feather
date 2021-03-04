//! Various block state values.
//! See the `libcraft-blocks` crate
//! for actual block definitions.

use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

/// Direction a block is facing in.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "snake_case")]
#[repr(u8)]
pub enum BlockFace {
    South,
    SouthSouthwest,
    Southwest,
    WestSouthwest,
    West,
    WestNorthwest,
    Northwest,
    NorthNorthwest,
    North,
    NorthNortheast,
    Northeast,
    EastNortheast,
    East,
    EastSoutheast,
    Southeast,
    SouthSoutheast,
}

/// Size of bamboo leaves.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum BambooLeaves {
    None,
    Small,
    Large,
}

/// Part of a bed.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum BedPart {
    Foot,
    Head,
}

/// How a bell is attached.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum BellAttachment {
    Ceiling,
    Floor,
    SingleWall,
    DoubleWall,
}

/// An axis. Used for bone blocks,
/// portal blocks, chains, etc.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Axis {
    X,
    Y,
    Z,
}

/// Block face a button or grindstone is attached to.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum AttachedFace {
    Ceiling,
    Floor,
    Wall,
}

/// Type of a chest.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ChestType {
    Single,
    /// Double chest; this block is on the left side.
    Left,
    /// Double chest; this block is on the right side.
    Right,
}

/// Which half of a door or flower block is.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum BlockHalf {
    Lower,
    Upper,
}

/// Which half of stairs.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum StairHalf {
    Bottom,
    Top,
}

/// To which side a door's hinge is.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum DoorHinge {
    Left,
    Right,
}

/// Orientation of a jigsaw block.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Orientation {
    DownEast,
    DownNorth,
    DownSouth,
    DownWest,
    EastUp,
    NorthUp,
    SouthUp,
    UpEast,
    UpNorth,
    UpSouth,
    UpWest,
    WestUp,
}

/// A note block instrument.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Instrument {
    Banjo,
    Basedrum,
    Bass,
    Bell,
    Bit,
    Chime,
    CowBell,
    Didgeridoo,
    Flute,
    Guitar,
    Harp,
    Hat,
    IronXylophone,
    Pling,
    Snare,
    Xylophone,
}

/// Type of a slab block.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum SlabType {
    Bottom,
    Top,
    Double,
}

/// Type of a moving piston or piston head.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum PistonType {
    Normal,
    Sticky,
}

/// Shape of a rail block.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum RailShape {
    EastWest,
    NorthEast,
    NorthSouth,
    NorthWest,
    SouthEast,
    SouthWest,
    AscendingEast,
    AscendingNorth,
    AscendingSouth,
    AscendingWest,
}

/// Mode of a redstone comparator.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ComparatorMode {
    Compare,
    Subtract,
}

/// How a redstone dust connects to a given side.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum RedstoneConnection {
    None,
    Side,
    Up,
}

/// Shape of a stairs block.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum StairShape {
    InnerLeft,
    InnerRight,
    OuterLeft,
    OuterRight,
    Straight,
}

/// Mode of a structure block.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum StructureBlockMode {
    Corner,
    Data,
    Load,
    Save,
}

/// How a wall connects to a given direction.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum WallConnection {
    None,
    Low,
    Tall,
}
