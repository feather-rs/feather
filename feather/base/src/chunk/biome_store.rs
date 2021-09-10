use libcraft_core::Biome;

use crate::{CHUNK_HEIGHT, CHUNK_WIDTH};

pub const BIOME_SAMPLE_RATE: usize = 4;

pub const BIOMES_PER_CHUNK: usize = (CHUNK_WIDTH / BIOME_SAMPLE_RATE)
    * (CHUNK_WIDTH / BIOME_SAMPLE_RATE)
    * (CHUNK_HEIGHT / BIOME_SAMPLE_RATE);

/// Stores the biomes of a chunk.
///
/// Since Minecraft 1.16, Mojang uses a 3D
/// biome grid sampled in 4x4x4 blocks. We
/// do the same, though right now Feather
/// only uses 2D biomes.
#[derive(Debug, Copy, Clone)]
pub struct BiomeStore {
    biomes: [Biome; BIOMES_PER_CHUNK],
}

impl BiomeStore {
    pub fn new(default_biome: Biome) -> Self {
        Self {
            biomes: [default_biome; BIOMES_PER_CHUNK],
        }
    }

    /// Creates a `BiomeStore` from a slice of `BIOMES_PER_CHUNK` biomes.
    ///
    /// Returns `None` if `biomes` is not of the correct length.
    pub fn from_slice(biome_slice: &[Biome]) -> Option<Self> {
        let mut biomes = [Biome::Plains; BIOMES_PER_CHUNK];
        if biomes.len() != biome_slice.len() {
            return None;
        }

        biomes.copy_from_slice(biome_slice);
        Some(Self { biomes })
    }

    /// Sets the biome at the given coordinates, in multiples
    /// of 4 blocks.
    ///
    /// # Panics
    /// Panics if `x >= 4`, `z >= 4`, or `y >= 64`.
    pub fn set(&mut self, x: usize, y: usize, z: usize, biome: Biome) {
        let index = self.index(x, y, z);
        self.biomes[index] = biome;
    }

    /// Gets the biome at the given coordinates,
    /// in multiples of 4 blocks.
    ///
    /// # Panics
    /// Panics if `x >= 4`, `z >= 4`, or `y >= 64`.
    pub fn get(&self, x: usize, y: usize, z: usize) -> Biome {
        let index = self.index(x, y, z);
        self.biomes[index]
    }

    pub fn get_at_block(&self, x: usize, y: usize, z: usize) -> Biome {
        self.get(x / 4, y / 4, z / 4)
    }

    /// Gets biome data as a raw slice.
    pub fn as_slice(&self) -> &[Biome] {
        &self.biomes
    }

    /// Gets biomes as a mutably slice.
    pub fn as_slice_mut(&mut self) -> &mut [Biome] {
        &mut self.biomes
    }

    fn index(&self, x: usize, y: usize, z: usize) -> usize {
        assert!(x < 4);
        assert!(y < 64);
        assert!(z < 4);
        x + (z * 4) + (y * 16)
    }
}

impl Default for BiomeStore {
    fn default() -> Self {
        Self::new(Biome::Plains)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_biome() {
        let biomes = BiomeStore::new(Biome::Badlands);
        for x in 0..4 {
            for y in 0..64 {
                for z in 0..4 {
                    assert_eq!(biomes.get(x, y, z), Biome::Badlands);
                }
            }
        }
    }

    #[test]
    fn set_and_get_biomes() {
        let mut biomes = BiomeStore::new(Biome::Beach);

        biomes.set(0, 1, 2, Biome::BambooJungle);
        assert_eq!(biomes.get(0, 1, 2), Biome::BambooJungle);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds() {
        let biomes = BiomeStore::new(Biome::Plains);
        biomes.get(4, 0, 0);
    }

    #[test]
    fn from_slice_correct_length() {
        let biome_slice = [Biome::BasaltDeltas; BIOMES_PER_CHUNK];
        let biomes = BiomeStore::from_slice(&biome_slice).unwrap();
        assert_eq!(biomes.as_slice(), biome_slice);
    }

    #[test]
    fn from_slice_incorrect_length() {
        let biome_slice = [Biome::BasaltDeltas; BIOMES_PER_CHUNK - 1];
        assert!(BiomeStore::from_slice(&biome_slice).is_none());
    }
}
