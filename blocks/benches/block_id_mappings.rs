#[macro_use]
extern crate criterion;

use criterion::{black_box, Criterion};
use feather_blocks::{BlockId, EastWire, WestWire};

// Some useless vanity benchmarks.

fn to_id_complex_state(c: &mut Criterion) {
    feather_blocks::init();
    let block = BlockId::redstone_wire()
        .with_east_wire(EastWire::Up)
        .with_power(15);
    c.bench_function("to_id_complex_state", |b| {
        b.iter(|| black_box(block).vanilla_id());
    });
}

fn from_id_complex_state(c: &mut Criterion) {
    feather_blocks::init();
    c.bench_function("from_id_complex_state", |b| {
        b.iter(|| BlockId::from_vanilla_id(black_box(7198)));
    });
}

fn update_properties_complex_state(c: &mut Criterion) {
    feather_blocks::init();
    c.bench_function("update_properties_complex_state", |b| {
        b.iter(|| {
            BlockId::redstone_wire()
                .with_east_wire(EastWire::Up)
                .with_west_wire(WestWire::Side)
                .with_power(15)
        });
    });
}

criterion_group!(
    benches,
    update_properties_complex_state,
    to_id_complex_state,
    from_id_complex_state
);
criterion_main!(benches);
