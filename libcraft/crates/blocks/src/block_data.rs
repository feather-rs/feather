use crate::data::RawBlockStateProperties;
use libcraft_core::block::{
    AttachedFace, Axis, BambooLeaves, BedPart, BellAttachment, BlockFace, BlockHalf, ChestType,
    ComparatorMode, Instrument, Orientation, PistonType, RailShape, SlabType, StairShape,
    StructureBlockMode, WallConnection,
};
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

/// Generalized BlockData structs

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

/// A block that has an "attached" property.
///
/// Updating the property will update tripwire
/// hook texture to indicate it is connected to
/// a string.
#[derive(Debug, BlockData)]
pub struct Attachable {
    pub attached: bool,
}

#[derive(Debug, BlockData)]
pub struct Bisected {
    pub half: BlockHalf,
}

/// Missing the implementation for retrieving the
/// valid faces of the block.
#[derive(Debug, BlockData)]
pub struct Directional {
    pub facing: BlockFace,
}

/// Represents the face to which a lever or
/// button is stuck.
#[derive(Debug, BlockData)]
pub struct FaceAttachable {
    pub attached_face: AttachedFace,
}

/// Represents the fluid level contained
/// within this block.
#[derive(Debug, BlockData)]
pub struct Levelled {
    /// Should be from 1 to 15, inclusive.
    pub level: u8,
}

#[derive(Debug, BlockData)]
pub struct Lightable {
    pub lit: bool,
}

/// Differs a bit from the spigot version but
/// can be used to set the faces
///
/// Missing a way
/// Also as of now, no way to retrieve the
/// "allowed faces"
#[derive(Debug, BlockData)]
pub struct MultipleFacing {
    pub down: bool,
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub west: bool,
    pub up: bool,
}

/// Denotes whether the block can be opened.
#[derive(Debug, BlockData)]
pub struct Openable {
    pub open: bool,
}

#[derive(Debug, BlockData)]
pub struct Orientable {
    pub axis: Axis,
}

/// Indicates whether block is in powered state
#[derive(Debug, BlockData)]
pub struct Powerable {
    pub powered: bool,
}

#[derive(Debug, BlockData)]
pub struct Rail {
    pub rail_shape: RailShape,
}

/// Current rotation of the block
#[derive(Debug, BlockData)]
pub struct Rotatable {
    pub rotation: BlockFace,
}

#[derive(Debug, BlockData)]
pub struct Snowable {
    pub snowy: bool,
}

/// Whether the block has water in it
#[derive(Debug, BlockData)]
pub struct Waterlogged {
    pub waterlogged: bool,
}

// Specific BlockData structs

// Bamboo (Ageable, Sapling)
#[derive(Debug, BlockData)]
pub struct Bamboo {
    pub age: u8,
    pub stage: u8,
    pub bamboo_leaves: BambooLeaves,
}

// Bed (Directional)
#[derive(Debug, BlockData)]
pub struct Bed {
    pub facing: BlockFace,
    pub part: BedPart,
}

// Beehive (Directional)
#[derive(Debug, BlockData)]
pub struct Beehive {
    pub facing: BlockFace,
    pub honey_level: u8,
}

// Bell (Directional, Powerable)
#[derive(Debug, BlockData)]
pub struct Bell {
    pub facing: BlockFace,
    pub powered: bool,
    pub bell_attachment: BellAttachment,
}

#[derive(Debug, BlockData)]
pub struct BrewingStand {
    pub has_bottle_0: bool,
    pub has_bottle_1: bool,
    pub has_bottle_2: bool,
}

#[derive(Debug, BlockData)]
pub struct BubbleColumn {
    pub drag: u8,
}

#[derive(Debug, BlockData)]
pub struct Cake {
    pub bites: u8,
}

// Campfire (Directional, Lightable, Waterlogged)
#[derive(Debug, BlockData)]
pub struct Campfire {
    pub facing: BlockFace,
    pub lit: bool,
    pub waterlogged: bool,
    pub signal_fire: bool,
}

// Chain (Orientable, Waterlogged)
#[derive(Debug, BlockData)]
pub struct Chain {
    pub facing: BlockFace,
    pub waterlogged: bool,
}

// Chest (Directional, Waterlogged)
#[derive(Debug, BlockData)]
pub struct Chest {
    pub facing: BlockFace,
    pub waterlogged: bool,
    pub chest_type: ChestType,
}

// Cocoa (Ageable, Directional)
#[derive(Debug, BlockData)]
pub struct Cocoa {
    pub age: u8,
    pub facing: BlockFace,
}

// CommandBlock (Directional)
#[derive(Debug, BlockData)]
pub struct CommandBlock {
    pub facing: BlockFace,
    pub conditional: bool,
}

// Comparator (Directional, Powerable)
#[derive(Debug, BlockData)]
pub struct Comparator {
    pub facing: BlockFace,
    pub powered: bool,
    pub comparator_mode: ComparatorMode,
}

// CoralWallFan (Directional, Waterlogged)
#[derive(Debug, BlockData)]
pub struct CoralWallFan {
    pub facing: BlockFace,
    pub waterlogged: bool,
}

// DaylightDetector (AnaloguePowerable)
#[derive(Debug, BlockData)]
pub struct DaylightDetector {
    pub power: u8,
    pub inverted: bool,
}

// Dispenser (Directional)
#[derive(Debug, BlockData)]
pub struct Dispenser {
    pub facing: BlockFace,
    pub triggered: bool,
}

// Door (Bisected, Directional, Openable, Powerable)
#[derive(Debug, BlockData)]
pub struct Door {
    pub half: BlockHalf,
    pub facing: BlockFace,
    pub open: bool,
    pub powered: bool,
}

// EnderChest (Directional, Waterlogged)
#[derive(Debug, BlockData)]
pub struct EnderChest {
    pub facing: BlockFace,
    pub waterlogged: bool,
}

// EndPortalFrame (Directional)
#[derive(Debug, BlockData)]
pub struct EndPortalFrame {
    pub facing: BlockFace,
    pub eye: bool,
}

#[derive(Debug, BlockData)]
pub struct Farmland {
    pub moisture: u8,
}

// Fence (MultipleFacing, Waterlogged)
/// Still requires the MultipleFacing implementation
#[derive(Debug, BlockData)]
pub struct Fence {
    pub waterlogged: bool,
}

// Fire (Ageable, MultipleFacing)
/// Still requires the MultipleFacing implementation
#[derive(Debug, BlockData)]
pub struct Fire {
    pub age: u8,
}

// Furnace (Directional, Lightable)
#[derive(Debug, BlockData)]
pub struct Furnace {
    pub facing: BlockFace,
    pub lit: bool,
}

// Gate (Directional, Openable, Powerable)
#[derive(Debug, BlockData)]
pub struct Gate {
    pub facing: BlockFace,
    pub open: bool,
    pub powered: bool,
    pub in_wall: bool,
}

// GlassPane (MultipleFacing, Waterlogged)
#[derive(Debug, BlockData)]
pub struct GlassPane {
    pub facing: BlockFace,
    pub waterlogged: bool,
}

// Grindstone (Directional, FaceAttachable)
#[derive(Debug, BlockData)]
pub struct Grindstone {
    pub facing: BlockFace,
    pub attached_face: AttachedFace,
}

// Hopper (Directional)
#[derive(Debug, BlockData)]
pub struct Hopper {
    pub facing: BlockFace,
    pub enabled: bool,
}

#[derive(Debug, BlockData)]
pub struct Jigsaw {
    pub orientation: Orientation,
}

#[derive(Debug, BlockData)]
pub struct JukeBox {
    pub has_record: bool,
}

// Ladder (Directional, Waterlogged)
#[derive(Debug, BlockData)]
pub struct Ladder {
    pub facing: BlockFace,
    pub waterlogged: bool,
}

// Lantern (Waterlogged)
#[derive(Debug, BlockData)]
pub struct Lantern {
    pub waterlogged: bool,
    pub hanging: bool,
}

#[derive(Debug, BlockData)]
pub struct Leaves {
    pub distance: u8,
    pub persistent: bool,
}

// Lectern (Directional, Powerable)
#[derive(Debug, BlockData)]
pub struct Lectern {
    pub facing: BlockFace,
    pub powered: bool,
    pub has_book: bool,
}

// NoteBlock (Powerable)
#[derive(Debug, BlockData)]
pub struct NoteBlock {
    pub powered: bool,
    pub instrument: Instrument,
}

// Observer (Directional, Powerable)
#[derive(Debug, BlockData)]
pub struct Observer {
    pub facing: BlockFace,
    pub powered: bool,
}

// Piston (Directional)
#[derive(Debug, BlockData)]
pub struct Piston {
    pub facing: BlockFace,
    pub extended: bool,
}

// PistonHead (TechnicalPiston)
#[derive(Debug, BlockData)]
pub struct PistonHead {
    pub facing: BlockFace,
    pub piston_type: PistonType,
    pub short: bool,
}

// RedstoneRail (Powerable, Rail)
#[derive(Debug, BlockData)]
pub struct RedstoneRail {
    pub powered: bool,
    pub rail_shape: RailShape,
}

// RedstoneWallTorch (Directional, Lightable)
#[derive(Debug, BlockData)]
pub struct RedstoneWallTorch {
    pub facing: BlockFace,
    pub lit: bool,
}

// RedstoneWire (AnaloguePowerable)
/// MISSING FUNCTIONALITY
#[derive(Debug, BlockData)]
pub struct RedstoneWire {
    pub power: u8,
    pub north: bool,
    pub east: bool,
    pub south: bool,
    pub west: bool,
}

// Repeater (Directional, Powerable)
pub struct Repeater {
    pub facing: BlockFace,
    pub powered: bool,
    pub delay: u8,
}

#[derive(Debug, BlockData)]
pub struct RespawnAnchor {
    pub charges: u8,
}

#[derive(Debug, BlockData)]
pub struct Sapling {
    pub stage: u8,
}

// Scaffolding (Waterlogged)
#[derive(Debug, BlockData)]
pub struct Scaffolding {
    pub waterlogged: bool,
    pub bottom: bool,
}

// SeaPickle (Waterlogged)
#[derive(Debug, BlockData)]
pub struct SeaPickle {
    pub waterlogged: bool,
    pub pickles: u8,
}

// Sign (Rotatable, Waterlogged)
#[derive(Debug, BlockData)]
pub struct Sign {
    pub rotation: BlockFace,
    pub waterlogged: bool,
}

// Slab (Waterlogged)
#[derive(Debug, BlockData)]
pub struct Slab {
    pub waterlogged: bool,
    pub slab_type: SlabType,
}

/// Lacks the min and max values for snow
#[derive(Debug, BlockData)]
pub struct Snow {
    pub layers: u8,
}

// Stairs (Bisected, Directional, Waterlogged)
#[derive(Debug, BlockData)]
pub struct Stairs {
    pub half: BlockHalf,
    pub facing: BlockFace,
    pub waterlogged: bool,
    pub stair_shape: StairShape,
}

#[derive(Debug, BlockData)]
pub struct StructureBlock {
    pub structure_block_mode: StructureBlockMode,
}

// Switch (Directional, FaceAttachable, Powerable)
#[derive(Debug, BlockData)]
pub struct Switch {
    pub facing: BlockFace,
    pub attached_face: AttachedFace,
    pub powered: bool,
}

// TechnicalPiston (Directional)
#[derive(Debug, BlockData)]
pub struct TechnicalPiston {
    pub facing: BlockFace,
    pub piston_type: PistonType,
}

#[derive(Debug, BlockData)]
pub struct TNT {
    pub unstable: bool,
}

// TrapDoor (Bisected, Directional, Openable, Powerable, Waterlogged)
#[derive(Debug, BlockData)]
pub struct TrapDoor {
    pub half: BlockHalf,
    pub facing: BlockFace,
    pub open: bool,
    pub powered: bool,
    pub waterlogged: bool,
}

// Tripwire (Attachable, MultipleFacing, Powerable)
/// MISSING MULTIPLEFACING
#[derive(Debug, BlockData)]
pub struct Tripwire {
    pub attached: bool,
    pub powered: bool,
    pub disarmed: bool,
}

// TripwireHook (Attachable, Directional, Powerable)
#[derive(Debug, BlockData)]
pub struct TripwireHook {
    pub attached: bool,
    pub facing: BlockFace,
    pub powered: bool,
}

#[derive(Debug, BlockData)]
pub struct TurtleEgg {
    pub hatch: u8,
    pub eggs: u8,
}

// Wall (Waterlogged)
#[derive(Debug, BlockData)]
pub struct Wall {
    pub waterlogged: bool,
    pub wall_north: WallConnection,
    pub wall_east: WallConnection,
    pub wall_south: WallConnection,
    pub wall_west: WallConnection,
    pub wall_up: WallConnection,
}

// WallSign (Directional, Waterlogged)
#[derive(Debug, BlockData)]
pub struct WallSign {
    pub facing: BlockFace,
    pub waterlogged: bool,
}

// https://hub.spigotmc.org/javadocs/spigot/org/bukkit/block/data/BlockData.html
