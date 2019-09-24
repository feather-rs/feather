//! Benchmarking of world generation. We benchmark both the generation
//! of an entire chunk and each separate stage of the composable generator.

#[macro_use]
extern crate criterion;

use criterion::{BenchmarkId, Criterion};
use feather_core::{Chunk, ChunkPosition};
use feather_server::worldgen::{
    BasicCompositionGenerator, BiomeGenerator, ComposableGenerator, CompositionGenerator,
    DensityMapGenerator, DensityMapGeneratorImpl, NearbyBiomes, TwoLevelBiomeGenerator,
    WorldGenerator,
};

const POSITIONS: [ChunkPosition; 2] = [
    ChunkPosition::new(0, 0),
    ChunkPosition::new(-2_000_000_000, 2_000_000_000),
];

const SEED: u64 = 78678354534;

pub fn generate_chunk(c: &mut Criterion) {
    let mut group = c.benchmark_group("generate_chunk");

    let generator = ComposableGenerator::default_with_seed(SEED);

    for position in POSITIONS.iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(*position),
            position,
            |b, position| b.iter(|| generator.generate_chunk(*position)),
        );
    }
}

pub fn generate_biome_grid(c: &mut Criterion) {
    let mut group = c.benchmark_group("generate_biome_grid");

    let generator = TwoLevelBiomeGenerator::default();

    for position in POSITIONS.iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(*position),
            position,
            |b, position| b.iter(|| generator.generate_for_chunk(*position, SEED)),
        );
    }
}

pub fn generate_density_map(c: &mut Criterion) {
    let mut group = c.benchmark_group("generate_density_map");

    let generator = DensityMapGeneratorImpl::default();

    for position in POSITIONS.iter() {
        let mut biomes = vec![];

        for z in -1..=1 {
            for x in -1..=1 {
                let pos = ChunkPosition::new(position.x + x, position.z + z);
                biomes.push(TwoLevelBiomeGenerator::default().generate_for_chunk(pos, SEED));
            }
        }

        let biomes = NearbyBiomes::from_vec(biomes);
        group.bench_with_input(
            BenchmarkId::from_parameter(*position),
            position,
            |b, position| b.iter(|| generator.generate_for_chunk(*position, &biomes, SEED)),
        );
    }
}

pub fn generate_composition(c: &mut Criterion) {
    let mut group = c.benchmark_group("generate_composition");

    let generator = BasicCompositionGenerator::default();

    for position in POSITIONS.iter() {
        let mut biomes = vec![];

        for z in -1..=1 {
            for x in -1..=1 {
                let pos = ChunkPosition::new(position.x + x, position.z + z);
                biomes.push(TwoLevelBiomeGenerator::default().generate_for_chunk(pos, SEED));
            }
        }

        let biomes = NearbyBiomes::from_vec(biomes);
        let density =
            DensityMapGeneratorImpl::default().generate_for_chunk(*position, &biomes, SEED);

        let mut chunk = Chunk::new(*position);

        group.bench_with_input(
            BenchmarkId::from_parameter(*position),
            position,
            |b, position| {
                b.iter(|| {
                    generator.generate_for_chunk(
                        &mut chunk,
                        *position,
                        &biomes.biomes[4],
                        &density,
                        SEED,
                    )
                })
            },
        );
    }
}

criterion_group!(
    benches,
    generate_chunk,
    generate_biome_grid,
    generate_density_map,
    generate_composition
);
criterion_main!(benches);
