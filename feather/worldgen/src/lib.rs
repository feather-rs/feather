#![forbid(unsafe_code)]

//! World generation for Feather.
//!
//! Generation is primarily based around the `ComposableGenerator`,
//! which allows configuration of a world generator pipeline.

use libcraft::biome::BiomeList;
pub use superflat::SuperflatWorldGenerator;

use libcraft::WorldHeight;
mod superflat;
pub struct VoidWorldGenerator;

/// Returns an index into a one-dimensional array
/// for the given x, y, and z values.
pub fn block_index(x: usize, y: i32, z: usize, world_height: WorldHeight, min_y: i32) -> usize {
    assert!(x < 16 && y >= min_y && y < min_y + *world_height as i32 && z < 16);
    (((y - min_y) as usize) << 8) | (x << 4) | z
}
