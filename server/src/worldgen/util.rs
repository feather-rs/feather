//! Utilities for world generation.

use feather_core::ChunkPosition;

/// Deterministically a seed for the given chunk. This allows
/// different seeds to be used for different chunk.
pub fn shuffle_seed_for_chunk(seed: u64, chunk: ChunkPosition) -> u64 {
    seed.wrapping_mul((chunk.x as u64).wrapping_add(1))
        .wrapping_add((chunk.z as u64).wrapping_add(1))
}

/// Deterministically shuffles a seed for the given chunk and chunk column.
pub fn shuffle_seed_for_column(seed: u64, chunk: ChunkPosition, col_x: usize, col_z: usize) -> u64 {
    shuffle_seed_for_chunk(seed, chunk)
        .wrapping_add(2)
        .wrapping_mul(((col_x as u64) << 4) + 4)
        .wrapping_mul(col_z as u64 + 4)
}

pub fn shuffle_seed_for_block(seed: u64, x: i32, z: i32) -> u64 {
    seed.wrapping_mul((x as u64).wrapping_mul(9324) ^ (z as u64).wrapping_shl(10))
}

pub fn map(x: f32, bmin: f32, bmax: f32, amin: f32, amax: f32) -> f32 {
    (x - bmin) * ((amax - amin) / (bmax - bmin)) + bmin
}
