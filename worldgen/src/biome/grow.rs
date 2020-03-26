pub mod continents;
pub mod grid;
pub mod zoom;

use crate::biome::grow::continents::ContinentsLayer;
use crate::biome::grow::zoom::ZoomLayer;
use crate::biome::{BiomeGenerator, Biomes};
use feather_core::{Biome, ChunkPosition};
pub use grid::Grid;

const CHUNK_WIDTH: i32 = feather_core::chunk::CHUNK_WIDTH as i32;

/// Applies some operation to the integer grid.
pub trait GridLayer: Send + Sync + 'static {
    fn generate(&self, seed: u64, x: i32, z: i32, size_x: i32, size_z: i32) -> Grid;
}

pub struct GrowBiomeGenerator {
    /// Recursive chain of biome operators.
    root: Box<dyn GridLayer>,
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

impl GrowBiomeGenerator {
    pub fn new() -> Self {
        let root = layer_chain! {
            ZoomLayer,
            ZoomLayer,
            ZoomLayer,
            ZoomLayer,
            ContinentsLayer
        };

        Self { root }
    }
}

impl BiomeGenerator for GrowBiomeGenerator {
    fn generate(&self, seed: u64, position: ChunkPosition) -> Biomes {
        let mut biomes = Biomes::default();

        let grid = self.root.generate(
            seed,
            position.x * CHUNK_WIDTH,
            position.z * CHUNK_WIDTH,
            CHUNK_WIDTH,
            CHUNK_WIDTH,
        );

        // Interpret final grid results as biome IDs.
        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_WIDTH {
                biomes[x as usize][z as usize] = Biome::from_protocol_id(i32::from(grid.at(x, z)))
                    .expect("biome generator returned invalid biome ID");
            }
        }

        biomes
    }
}
