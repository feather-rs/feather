//! Implements a density map generator using 3D Perlin noise.
//!
//! Over the 2D height map generator, this has the advantage that terrain
//! is more interesting; overhangs and the like will be able to generate.

use crate::{block_index, noise, DensityMapGenerator, NearbyBiomes, NoiseLerper};
use base::{Biome, ChunkPosition};
use bitvec::order::LocalBits;
use bitvec::vec::BitVec;
use once_cell::sync::Lazy;
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
    fn generate_for_chunk(
        &self,
        chunk: ChunkPosition,
        biomes: &NearbyBiomes,
        seed: u64,
    ) -> BitVec<LocalBits, u8> {
        let mut density = BitVec::from_vec(vec![0u8; 16 * 256 * 16 / 8]);

        let uninterpolated_densities = generate_density(chunk, biomes, seed);
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
fn generate_density(chunk: ChunkPosition, biomes: &NearbyBiomes, seed: u64) -> Vec<f32> {
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
    // Additional 2D height noise for extra detail.
    let height_noise = NoiseBuilder::fbm_2d_offset(x_offset, len, z_offset, len)
        .with_seed(noise_seed + 3)
        .with_octaves(2)
        .with_freq(0.08)
        .generate()
        .0;

    let mut result = vec![0.0; DENSITY_WIDTH * DENSITY_HEIGHT * DENSITY_WIDTH];

    // Loop through subchunks and generate density for each.
    for subx in 0..DENSITY_WIDTH {
        for subz in 0..DENSITY_WIDTH {
            // TODO: average nearby biome parameters
            let (amplitude, midpoint) = column_parameters(biomes, subx, subz);

            let height = height_noise[(subz * len) + subx] * 3.0;

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
                result[index] =
                    lerp(density_1, density_2, choice) * 0.2 + height_offset * 2.0 + height;
            }
        }
    }

    result
}

/// Elevation height field, used to weight
/// the averaging of nearby biome heights.
static ELEVATION_WEIGHT: Lazy<[[f32; 19]; 19]> = Lazy::new(|| {
    let mut array = [[0.0; 19]; 19];
    for (x, values) in array.iter_mut().enumerate() {
        for (z, value) in values.iter_mut().enumerate() {
            let mut x_squared = x as i32 - 9;
            x_squared *= x_squared;
            let mut z_sqaured = z as i32 - 9;
            z_sqaured *= z_sqaured;
            *value = 10.0 / (x_squared as f32 + z_sqaured as f32 + 0.2).sqrt();
        }
    }
    array
});

/// Computes the target amplitude and midpoint for the
/// given column, using a 9x9 grid of biomes
/// around the column to determine a weighted average.
///
/// The X and Z parameters are the coordinates of the subchunk
/// within the chunk, not the block coordinate.
fn column_parameters(biomes: &NearbyBiomes, x: usize, z: usize) -> (f32, f32) {
    let x = x as i32 * (DENSITY_WIDTH as i32 - 1);
    let z = z as i32 * (DENSITY_WIDTH as i32 - 1);

    let mut sum_amplitudes = 0.0;
    let mut sum_midpoints = 0.0;
    let mut sum_weights = 0.0;

    // Loop through columns in 9x9 grid and compute weighted average of amplitudes
    // and midpoints.
    for block_x in -9..=9 {
        for block_z in -9..=9 {
            let abs_x = x + block_x;
            let abs_z = z + block_z;

            let biome = biomes.get_at_block(abs_x, 0, abs_z);
            let (amplitude, midpoint) = biome_parameters(biome);

            let weight = ELEVATION_WEIGHT[(block_x + 9) as usize][(block_z + 9) as usize];

            sum_amplitudes += amplitude * weight;
            sum_midpoints += midpoint * weight;
            sum_weights += weight;
        }
    }

    sum_amplitudes /= sum_weights;
    sum_midpoints /= sum_weights;

    (sum_amplitudes, sum_midpoints)
}

/// Returns the amplitude and midpoint for a given biome
/// type as a tuple in that order.
///
/// All original values were taken from Cuberite's source,
/// so all credit for this function goes to their team
/// for the presumably highly laborious effort
/// involved in finding these values.
fn biome_parameters(biome: Biome) -> (f32, f32) {
    match biome {
        Biome::Beach => (0.2, 60.0),
        Biome::BirchForest => (0.1, 64.0),
        Biome::BirchForestHills => (0.075, 64.0),
        Biome::TallBirchHills => (0.075, 68.0),
        Biome::TallBirchForest => (0.1, 64.0),
        Biome::SnowyBeach => (0.3, 62.0),
        Biome::Taiga => (0.3, 62.0),
        Biome::TaigaHills => (0.075, 68.0),
        Biome::DesertHills => (0.075, 68.0),
        Biome::DeepOcean => (0.17, 35.0),
        Biome::Desert => (0.15, 62.0),
        Biome::Mountains => (0.045, 75.0),
        Biome::MountainEdge => (0.1, 70.0),
        Biome::WoodedMountains => (0.04, 80.0),
        Biome::FlowerForest => (0.1, 64.0),
        Biome::Forest => (0.1, 64.0),
        Biome::Jungle => (0.1, 63.0),
        Biome::Ocean => (0.12, 45.0),
        Biome::Plains => (0.3, 62.0),
        Biome::Savanna => (0.3, 62.0),
        Biome::SavannaPlateau => (0.3, 85.0),
        Biome::StoneShore => (0.075, 60.0),
        Biome::SunflowerPlains => (0.3, 62.0),
        Biome::Swamp => (0.25, 59.0),
        // TODO: finish this list
        _ => (0.3, 62.0),
    }
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
