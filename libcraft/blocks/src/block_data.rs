use crate::data::{RawBlockStateProperties, ValidProperties};
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
    fn from_raw(raw: &RawBlockStateProperties, valid: &'static ValidProperties) -> Option<Self>
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
    age: u8,
    valid_properties: &'static ValidProperties,
}

/// A block that can be powered with a redstone
/// signal.
#[derive(Debug, BlockData)]
pub struct AnaloguePowerable {
    power: u8,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Attachable {
    attached: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Bisected {
    half: BlockHalf,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Directional {
    facing: BlockFace,
    valid_properties: &'static ValidProperties,
}

/// Represents the face to which a lever or
/// button is stuck.
#[derive(Debug, BlockData)]
pub struct FaceAttachable {
    attached_face: AttachedFace,
    valid_properties: &'static ValidProperties,
}

/// Represents the fluid level contained
/// within this block.
#[derive(Debug, BlockData)]
pub struct Levelled {
    level: u8,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Lightable {
    lit: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct MultipleFacing {
    down: bool,
    east: bool,
    north: bool,
    south: bool,
    west: bool,
    up: bool,
    valid_properties: &'static ValidProperties,
}

/// Denotes whether the block can be opened.
#[derive(Debug, BlockData)]
pub struct Openable {
    open: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Orientable {
    axis: Axis,
    valid_properties: &'static ValidProperties,
}

/// Indicates whether block is in powered state
#[derive(Debug, BlockData)]
pub struct Powerable {
    powered: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Rail {
    rail_shape: RailShape,
    valid_properties: &'static ValidProperties,
}

/// Current rotation of the block
#[derive(Debug, BlockData)]
pub struct Rotatable {
    rotation: BlockFace,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Snowable {
    snowy: bool,
    valid_properties: &'static ValidProperties,
}

/// Whether the block has water in it
#[derive(Debug, BlockData)]
pub struct Waterlogged {
    waterlogged: bool,
    valid_properties: &'static ValidProperties,
}

// Specific BlockData structs

#[derive(Debug, BlockData)]
pub struct Bamboo {
    age: u8,
    stage: u8,
    bamboo_leaves: BambooLeaves,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Bed {
    facing: BlockFace,
    part: BedPart,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Beehive {
    facing: BlockFace,
    honey_level: u8,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Bell {
    facing: BlockFace,
    powered: bool,
    bell_attachment: BellAttachment,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct BrewingStand {
    has_bottle_0: bool,
    has_bottle_1: bool,
    has_bottle_2: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct BubbleColumn {
    drag: u8,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Cake {
    bites: u8,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Campfire {
    facing: BlockFace,
    lit: bool,
    waterlogged: bool,
    signal_fire: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Chain {
    facing: BlockFace,
    waterlogged: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Chest {
    facing: BlockFace,
    waterlogged: bool,
    chest_type: ChestType,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Cocoa {
    age: u8,
    facing: BlockFace,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct CommandBlock {
    facing: BlockFace,
    conditional: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Comparator {
    facing: BlockFace,
    powered: bool,
    comparator_mode: ComparatorMode,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct CoralWallFan {
    facing: BlockFace,
    waterlogged: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct DaylightDetector {
    power: u8,
    inverted: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Dispenser {
    facing: BlockFace,
    triggered: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Door {
    half: BlockHalf,
    facing: BlockFace,
    open: bool,
    powered: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct EndPortalFrame {
    facing: BlockFace,
    eye: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Farmland {
    moisture: u8,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Fence {
    waterlogged: bool,
    down: bool,
    east: bool,
    north: bool,
    south: bool,
    west: bool,
    up: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Furnace {
    facing: BlockFace,
    lit: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Gate {
    facing: BlockFace,
    open: bool,
    powered: bool,
    in_wall: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct GlassPane {
    facing: BlockFace,
    waterlogged: bool,
    down: bool,
    east: bool,
    north: bool,
    south: bool,
    west: bool,
    up: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Grindstone {
    facing: BlockFace,
    attached_face: AttachedFace,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Hopper {
    facing: BlockFace,
    enabled: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Jigsaw {
    orientation: Orientation,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct JukeBox {
    has_record: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Ladder {
    facing: BlockFace,
    waterlogged: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Lantern {
    waterlogged: bool,
    hanging: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Leaves {
    distance: u8,
    persistent: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Lectern {
    facing: BlockFace,
    powered: bool,
    has_book: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct NoteBlock {
    powered: bool,
    instrument: Instrument,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Observer {
    facing: BlockFace,
    powered: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Piston {
    facing: BlockFace,
    extended: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct PistonHead {
    facing: BlockFace,
    piston_type: PistonType,
    short: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct RedstoneRail {
    powered: bool,
    rail_shape: RailShape,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct RedstoneWallTorch {
    facing: BlockFace,
    lit: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct RedstoneWire {
    power: u8,
    north: bool,
    east: bool,
    south: bool,
    west: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Repeater {
    facing: BlockFace,
    powered: bool,
    delay: u8,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct RespawnAnchor {
    charges: u8,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Sapling {
    stage: u8,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Scaffolding {
    waterlogged: bool,
    bottom: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct SeaPickle {
    waterlogged: bool,
    pickles: u8,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Sign {
    rotation: BlockFace,
    waterlogged: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Slab {
    waterlogged: bool,
    slab_type: SlabType,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Snow {
    layers: u8,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Stairs {
    half: BlockHalf,
    facing: BlockFace,
    waterlogged: bool,
    stair_shape: StairShape,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct StructureBlock {
    structure_block_mode: StructureBlockMode,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Switch {
    facing: BlockFace,
    attached_face: AttachedFace,
    powered: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct TechnicalPiston {
    facing: BlockFace,
    piston_type: PistonType,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Tnt {
    unstable: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct TrapDoor {
    half: BlockHalf,
    facing: BlockFace,
    open: bool,
    powered: bool,
    waterlogged: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Tripwire {
    attached: bool,
    powered: bool,
    down: bool,
    east: bool,
    north: bool,
    south: bool,
    west: bool,
    up: bool,
    disarmed: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct TripwireHook {
    attached: bool,
    facing: BlockFace,
    powered: bool,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct TurtleEgg {
    hatch: u8,
    eggs: u8,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct Wall {
    waterlogged: bool,
    wall_north: WallConnection,
    wall_east: WallConnection,
    wall_south: WallConnection,
    wall_west: WallConnection,
    wall_up: WallConnection,
    valid_properties: &'static ValidProperties,
}

#[derive(Debug, BlockData)]
pub struct WallSign {
    facing: BlockFace,
    waterlogged: bool,
    valid_properties: &'static ValidProperties,
}

// https://hub.spigotmc.org/javadocs/spigot/org/bukkit/block/data/BlockData.html
