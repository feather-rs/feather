use feather_core::chunk::CHUNK_WIDTH;
use feather_core::{Biome, ChunkPosition};
use std::ops::{Deref, DerefMut};

pub mod grow;

/// Stores the chunks of a biome.
#[derive(Debug, Clone)]
pub struct Biomes([[Biome; CHUNK_WIDTH]; CHUNK_WIDTH]);

impl Deref for Biomes {
    type Target = [[Biome; CHUNK_WIDTH]];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Biomes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for Biomes {
    fn default() -> Self {
        Self([[Biome::Plains; CHUNK_WIDTH]; CHUNK_WIDTH])
    }
}

/// Implemented by biome generators.
pub trait BiomeGenerator: Send + Sync + 'static {
    /// Generates a biome grid for the given chunk.
    fn generate(&self, seed: u64, position: ChunkPosition) -> Biomes;
}
