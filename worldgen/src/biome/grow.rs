pub mod continents;
pub mod grid;
pub mod smooth;
pub mod zoom;

use crate::biome::grow::continents::ContinentsLayer;
use crate::biome::grow::zoom::ZoomLayer;
use crate::biome::{BiomeGenerator, Biomes};
use feather_core::ChunkPosition;
pub use grid::Grid;
use rand::Rng;
use smooth::SmoothLayer;

const CHUNK_WIDTH: i32 = feather_core::chunk::CHUNK_WIDTH as i32;

/// Applies some operation to the integer grid.
pub trait GridLayer: Send + Sync + 'static {
    fn generate(&self, seed: u64, x: i32, z: i32, size_x: i32, size_z: i32) -> Grid;
}

pub struct GrowBiomeGenerator {
    /// Recursive chain of biome operators.
    pub root: Box<dyn GridLayer>,
}

macro_rules! layer_chain {
    ($first:ident, $($generator:ident),+) => {
      {
        Box::new($first {
            below: layer_chain!($($generator ),*),
        })
      }
    };
    ($first:ident) => {
        {
            Box::new($first)
        }
    }
}

impl Default for GrowBiomeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl GrowBiomeGenerator {
    pub fn new() -> Self {
        let root = layer_chain! {
            SmoothLayer,
            ZoomLayer,
            ZoomLayer,
            SmoothLayer,
            ZoomLayer,
            ZoomLayer,
            SmoothLayer,
            ZoomLayer,
            ZoomLayer,
            SmoothLayer,
            ZoomLayer,
            ZoomLayer,
            ContinentsLayer
        };

        Self { root }
    }
}

impl BiomeGenerator for GrowBiomeGenerator {
    fn generate(&self, seed: u64, position: ChunkPosition) -> Biomes {
        let biomes = Biomes::default();

        let _grid = self.root.generate(
            seed,
            position.x * CHUNK_WIDTH,
            position.z * CHUNK_WIDTH,
            CHUNK_WIDTH,
            CHUNK_WIDTH,
        );

        // Interpret final grid results as biome IDs.

        biomes
    }
}

pub fn select<T>(rng: &mut impl Rng, values: &[T]) -> T
where
    T: Copy,
{
    let index = rng.gen_range(0, values.len());
    values[index]
}
