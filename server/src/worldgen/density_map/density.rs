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

                    let is_solid = value < 0.0;
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
/// This is based on Cuberite's implementation of the same function.
/// It works by having two 3D density noises, with another noise
/// to interpolate between the values of each. It then uses
/// a vertical linear gradient to determine densities of the
/// subchunks at each given Y level for a column.
///
/// # Notes
/// The density values emitted from this function should
/// be considered solid if less than 0 and air if greater
/// than 0. This is contrary to what might seem logical.
fn generate_density(chunk: ChunkPosition, seed: u64) -> Vec<f32> {
    // TODO: generate based on biome

    let x_offset = (chunk.x * (DENSITY_WIDTH as i32 - 1)) as f32;
    let y_offset = 0.0;
    let z_offset = (chunk.z * (DENSITY_WIDTH as i32 - 1)) as f32;
    let len = DENSITY_WIDTH;
    let height = DENSITY_HEIGHT;

    let noise_seed = seed as i32;

    // Generate various noises.
    let choice_noise = NoiseBuilder::fbm_3d_offset(x_offset, len, y_offset, height, z_offset, len)
        .with_seed(noise_seed)
        .with_octaves(2)
        .with_freq(0.001)
        .generate()
        .0;
    let density_noise_1 =
        NoiseBuilder::fbm_3d_offset(x_offset, len, y_offset, height, z_offset, len)
            .with_seed(noise_seed + 1)
            .with_octaves(2)
            .with_freq(0.2)
            .generate()
            .0;
    let density_noise_2 =
        NoiseBuilder::fbm_3d_offset(x_offset, len, y_offset, height, z_offset, len)
            .with_seed(noise_seed + 2)
            .with_octaves(2)
            .with_freq(0.2)
            .generate()
            .0;
    // TODO: additional 2D height noise

    let mut result = vec![0.0; DENSITY_WIDTH * DENSITY_HEIGHT * DENSITY_WIDTH];

    // Loop through subchunks and generate density for each.
    for subx in 0..DENSITY_WIDTH {
        for subz in 0..DENSITY_WIDTH {
            // TODO: base this on nearby biomes.
            let amplitude = 0.075f32;
            let midpoint = 68.0f32;

            // Loop through Y axis of this subchunk column.
            for suby in 0..DENSITY_HEIGHT {
                // Linear gradient used to offset based on height.
                let mut height_offset = ((suby as f32 * 8.0) - midpoint) * amplitude;

                // If we are below the midpoint, increase the slope of the gradient.
                // This creates smoother terrain.
                if height_offset < 0.0 {
                    height_offset *= 4.0;
                }

                // When we are near sky limit, decrease
                // the slope. This ensures that very tall
                // mountains don't artificially cut off
                // at Y=256.
                if suby > 26 {
                    height_offset += (suby as f32 - 28.0) / 4.0;
                }

                let index = DENSITY_WIDTH * suby + subx + DENSITY_WIDTH * DENSITY_HEIGHT * subz;

                let choice = choice_noise[index] * 100.0;
                let density_1 = density_noise_1[index] * 50.0;
                let density_2 = density_noise_2[index] * 50.0;

                // Average between two density values based on choice weight.
                result[index] = lerp(density_1, density_2, choice) + height_offset;
            }
        }
    }

    result
}

/// Interpolates between two values based on the given
/// weight.
///
/// The weight is clamped to [0.0, 1.0].
fn lerp(a: f32, b: f32, weight: f32) -> f32 {
    if weight < 0.0 {
        return a;
    } else if weight > 1.0 {
        return b;
    }

    a + (b - a) * weight
}
