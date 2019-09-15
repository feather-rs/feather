//! Generation of tree schematics.

use super::TreeType;
use crate::worldgen::schematic::{Schematic, SchematicBuilder};
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

/// Generates a tree with the given type and seed.
pub fn generate_tree(ty: TreeType, seed: u64) -> Schematic {
    let mut rng = XorShiftRng::seed_from_u64(seed);
    match ty {
        TreeType::Oak => generate_oak(&mut rng),
        TreeType::DarkOak => generate_dark_oak(&mut rng),
        TreeType::Birch => generate_birch(&mut rng),
        TreeType::Spruce => generate_spruce(&mut rng),
        TreeType::Acacia => generate_acacia(&mut rng),
        TreeType::Jungle => generate_jungle(&mut rng),
    }
}

fn generate_oak<R: Rng>(rng: &mut R) -> Schematic {
    let ty = TreeType::Oak;
    // TODO: tall oak variant
    let mut tree = SchematicBuilder::new()
        .with_dimensions(5, 7, 5)
        .with_center(2, 0, 2)
        .build();

    // Logs
    for y in 0..6 {
        tree.set_block_at(0, y, 0, TreeType::Oak.log_block());
    }

    // Leaves

    // Bottom part (two 5x5 squares with random corners)
    for x in -2isize..=2 {
        for z in -2isize..=2 {
            for y in 0..2 {
                let y = y + 3;

                if x.abs() == 2 && z.abs() == 2 && rng.gen_bool(0.7) {
                    continue; // Variance in corner leaves
                }

                tree.set_block_at(x, y, z, ty.leaf_block(manhattan(0, y, 0, x, y, z) as i32));
            }
        }
    }
    // Top part
    let positions = [[1, 0], [-1, 0], [0, 1], [0, -1]];
    for y in 5..=6 {
        positions
            .iter()
            .for_each(|pos| tree.set_block_at(pos[0], y, pos[1], ty.leaf_block(1)));
    }
    tree.set_block_at(0, 6, 0, ty.leaf_block(1));

    tree
}

fn generate_dark_oak<R: Rng>(_rng: &mut R) -> Schematic {
    unimplemented!()
}

fn generate_birch<R: Rng>(_rng: &mut R) -> Schematic {
    unimplemented!()
}

fn generate_spruce<R: Rng>(_rng: &mut R) -> Schematic {
    unimplemented!()
}

fn generate_acacia<R: Rng>(_rng: &mut R) -> Schematic {
    unimplemented!()
}

fn generate_jungle<R: Rng>(_rng: &mut R) -> Schematic {
    unimplemented!()
}

fn manhattan(x1: isize, y1: isize, z1: isize, x2: isize, y2: isize, z2: isize) -> isize {
    (x2 - x1).abs() + (y2 - y1).abs() + (z2 - z1).abs()
}
