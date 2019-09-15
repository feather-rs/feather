//! Tree generation.

mod schematic;
mod ty;

use crate::worldgen::presence::PresenceGrid;
use crate::worldgen::util::shuffle_seed_for_column;
use crate::worldgen::{FinishingGenerator, NearbyBiomes, TopBlocks};
use feather_core::{Biome, Chunk, ChunkPosition};
use rand::distributions::{Distribution, WeightedIndex};
use rand::{Rng, RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::cmp;
use std::convert::TryFrom;
pub use ty::TreeType;

/// Finisher for generating trees, depending on biome.
#[derive(Default)]
pub struct TreeFinisher;

impl FinishingGenerator for TreeFinisher {
    fn generate_for_chunk(
        &self,
        chunk: &mut Chunk,
        biomes: &NearbyBiomes,
        top_blocks: &TopBlocks,
        seed: u64,
    ) {
        // Presence grid for trees.
        // We set values to `true` for
        // any column in which a tree will be generated.
        //let mut tree_presences = PresenceGrid::new();

        // Compute presence of trees for each column within this chunk
        // and the 6 columns bordering it on each side. For columns with
        // trees generated, do the following:
        // * First, check if a tree is already generated within params.spread.
        // If so, remove the tree.
        // * Generate a tree schematic for the tree and apply it to the chunk.
        for x in -6..16 + 6 {
            for z in -6..16 + 6 {
                let biome = biomes.biome_at(x, z);
                let params = params_for_biome(biome);

                if compute_presence_for_column(chunk.position(), &params, x, z, seed) {
                    /*if tree_presences.is_presence_within(x, z, params.spread) {
                        continue;
                    }

                    tree_presences.set_presence_at(x, z, true);*/

                    let column_seed = column_seed(seed, chunk.position(), x, z);
                    let mut rng = XorShiftRng::seed_from_u64(column_seed);

                    // Choose tree type based on weights.
                    let possible_trees = params.possible_trees;
                    let index = possible_trees.weights.sample(&mut rng);
                    let tree = possible_trees.trees[index];

                    // Generate tree schematic and write it to the chunk.
                    let schematic = schematic::generate_tree(tree, seed);
                    let y = top_blocks.top_block_at(
                        cmp::min(15, usize::try_from(x).unwrap_or(0)),
                        cmp::min(15, usize::try_from(z).unwrap_or(0)),
                    ); // TODO
                    schematic.write_to_chunk(
                        chunk,
                        (chunk.position().x * 16) as isize + x,
                        y as isize + 1,
                        (chunk.position().z * 16) as isize + z,
                    );
                }
            }
        }
    }
}

fn compute_presence_for_column(
    center_chunk: ChunkPosition,
    params: &TreeParams,
    column_x: isize,
    column_z: isize,
    seed: u64,
) -> bool {
    let seed = column_seed(seed, center_chunk, column_x, column_z);
    let mut rng = XorShiftRng::seed_from_u64(seed);

    rng.gen_bool(params.frequency / 256.0)
}

fn column_seed(seed: u64, center_chunk: ChunkPosition, column_x: isize, column_z: isize) -> u64 {
    let abs_x = center_chunk.x * 16 + column_x as i32;
    let abs_z = center_chunk.z * 16 + column_z as i32;

    seed.wrapping_mul(((abs_x as u64) << 32) | abs_z as u64)
}

/// WeightedIndex for a biome's tree weights, with
/// the array of possible tree types corresponding
/// to samples from the weighted index.
#[derive(Debug)]
struct TreeWeights {
    weights: WeightedIndex<f64>,
    trees: &'static [TreeType],
}

impl TreeWeights {
    fn new(weights: &[f64], trees: &'static [TreeType]) -> Self {
        Self {
            weights: WeightedIndex::new(weights).unwrap(),
            trees,
        }
    }
}

/// Settings of a biome for tree generation.
#[derive(Debug)]
struct TreeParams {
    /// Frequency of trees. Higher values mean more trees.
    frequency: f64,
    /// Minimum distance between two trees.
    spread: u32,
    /// Table of tree types possible for this biome
    /// and their corresponding weights. The table
    /// for each tree is in a `lazy_static` variable,
    /// since initializing it requires an expensive
    /// allocation.
    possible_trees: &'static TreeWeights,
}

lazy_static! {
    static ref WEIGHTS_DEFAULT: TreeWeights = TreeWeights::new(&[1.0], &[TreeType::Oak]);
    static ref WEIGHTS_FOREST: TreeWeights = TreeWeights::new(&[1.0], &[TreeType::Oak]);
}

impl Default for TreeParams {
    /// Tree parameters which generate zero trees.
    fn default() -> Self {
        Self {
            frequency: 0.0,
            spread: 0,
            possible_trees: &WEIGHTS_DEFAULT,
        }
    }
}

/// Returns the tree parameters for the given biome.
fn params_for_biome(biome: Biome) -> TreeParams {
    match biome {
        Biome::Forest | Biome::DarkForest => TreeParams {
            frequency: 4.0,
            spread: 2,
            possible_trees: &WEIGHTS_FOREST,
        },
        _ => TreeParams::default(), // TODO
    }
}
