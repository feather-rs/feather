#[macro_use]
extern crate criterion;

use criterion::{black_box, Criterion};
use feather_blocks::{
    Block, BlockExt, RedSandstoneStairsData, RedSandstoneStairsFacing, RedSandstoneStairsHalf,
    RedSandstoneStairsShape,
};

fn to_id_complex_state(c: &mut Criterion) {
    c.bench_function("to_id_complex_state", |b| {
        b.iter(|| {
            black_box(Block::RedSandstoneStairs(RedSandstoneStairsData {
                waterlogged: true,
                half: RedSandstoneStairsHalf::Top,
                shape: RedSandstoneStairsShape::Straight,
                facing: RedSandstoneStairsFacing::South,
            }))
            .native_state_id()
        });
    });
}

fn from_id_complex_state(c: &mut Criterion) {
    c.bench_function("from_id_complex_state", |b| {
        b.iter(|| Block::from_native_state_id(black_box(7198)));
    });
}

criterion_group!(benches, to_id_complex_state, from_id_complex_state);
criterion_main!(benches);
