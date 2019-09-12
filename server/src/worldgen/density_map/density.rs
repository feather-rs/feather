//! Implements a density map generator using 3D Perlin noise.
//!
//! Over the 2D height map generator, this has the advantage that terrain
//! is more interesting; overhangs and the like will be able to generate.

use crate::worldgen::{block_index, noise, ChunkBiomes, DensityMapGenerator, NoiseLerper};
use bitvec::vec::BitVec;
use feather_core::ChunkPosition;
use simdnoise::NoiseBuilder;

/// A density map generator using 3D Perlin noise.
///
/// This generator should be used over the height map generator
/// when seeking correct-looking worlds;
///
/// # Implementation
/// Density calculation works as follows:
/// * Generate a base 3D Perlin nosie with settings depending
/// on the biome. Use linear interpolation on noise (this
/// is handled by `Wrapped3DPerlinNoise`)`.
/// * Depending on the density value from the noise, decide
/// whether the position is solid or air.
#[derive(Debug, Default)]
pub struct DensityMapGeneratorImpl;

impl DensityMapGenerator for DensityMapGeneratorImpl {
    fn generate_for_chunk(&self, chunk: ChunkPosition, _biomes: &ChunkBiomes, seed: u64) -> BitVec {
        let mut density = bitvec![0; 16 * 256 * 16];

        let uninterpolated_densities = generate_density(chunk, seed);
        let noise = NoiseLerper::new(&uninterpolated_densities)
            .with_offset(chunk.x, chunk.z)
            .generate();

        for x in 0..16 {
            for y in 0..256 {
                for z in 0..16 {
                    let value = noise[noise::index(x, y, z)];

                    let is_solid = value > 0.0;
                    let index = block_index(x, y, z);
                    density.set(index, is_solid);
                }
            }
        }

        density
    }
}

const DENSITY_WIDTH: usize = 5;
const DENSITY_HEIGHT: usize = 33;

/// Generates a 5x33x5 density array to pass to `NoiseLerper`.
///
/// This is based on Glowstone's OverworldGenerator.generateTerrainDensity().
/// Much of the credit for this work goes to them.
fn generate_density(chunk: ChunkPosition, seed: u64) -> Vec<f32> {
    // Rather than a single 3D density noise,
    // we use a 2D height noise with 3D roughness/detail
    // noises. This has the following advantages:
    // * Interesting terrain with overhangs and such can
    // still be generated thanks to the roughness/detail
    // noises.
    // * We can interpolate between biome height values
    // because of the 2D height map, something which wouldn't
    // be possible using a 3D density noise.
    let mut densities = vec![0.0; DENSITY_WIDTH * DENSITY_HEIGHT * DENSITY_WIDTH];

    let x_offset = (chunk.x * (DENSITY_WIDTH as i32 - 1)) as f32;
    let z_offset = (chunk.z * (DENSITY_WIDTH as i32 - 1)) as f32;
    let noise_seed = seed as i32;

    // Various noises.
    // Height: base height of each column.
    // Detail and roughness: 3D noises to add detail to base height.
    let height = NoiseBuilder::fbm_2d_offset(x_offset, DENSITY_WIDTH, z_offset, DENSITY_WIDTH)
        .with_seed(noise_seed)
        .with_octaves(16)
        .generate_scaled(0.0, 1.0);
    let roughness = NoiseBuilder::fbm_3d_offset(
        x_offset,
        DENSITY_WIDTH,
        0.0,
        DENSITY_HEIGHT,
        z_offset,
        DENSITY_WIDTH,
    )
    .with_freq(0.1)
    .with_seed(noise_seed)
    .with_octaves(16)
    .generate_scaled(0.0, 1.0);
    let detail = NoiseBuilder::fbm_3d_offset(
        x_offset,
        DENSITY_WIDTH,
        0.0,
        DENSITY_HEIGHT,
        z_offset,
        DENSITY_WIDTH,
    )
    .with_seed(noise_seed)
    .with_octaves(8)
    .with_freq(0.1)
    .generate_scaled(0.0, 1.0);

    // Current index into the height noise array.
    let mut height_index = 0;
    // Current index into the 3D noise arrays.
    let mut index = 0;

    // Iterate through each subchunk in the chunk.
    // A subchunk is 4x32x4 blocks.
    for subx in 0..DENSITY_WIDTH {
        for subz in 0..DENSITY_WIDTH {
            // TODO: interpolate between biome height values.

            // Modify height value with a bunch of magic
            // numbers. This may need some tweaking.
            let mut height = height[height_index];
            height_index += 1;

            // Traverse the Y axis and compute density values for each
            // subchunk.
            for suby in 0..DENSITY_HEIGHT {
                let height = height - suby as f32 + (64.0 / suby as f32);
                let density = height /*+ roughness[index] /  4.0 + detail[index] / 8.0*/;

                densities
                    [(suby * (DENSITY_WIDTH * DENSITY_WIDTH) + (subz * DENSITY_WIDTH) + subx)] =
                    density;

                index += 1;
            }
        }
    }

    densities
}

#[allow(clippy::float_cmp)]
fn max(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

#[allow(clippy::float_cmp)]
fn min(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}
