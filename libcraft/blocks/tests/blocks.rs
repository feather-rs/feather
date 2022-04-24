use std::time::Instant;

use libcraft_blocks::{block_data::Ageable, BlockKind, BlockState};

#[test]
fn update_block_data() {
    let start = Instant::now();

    let mut block = BlockState::from_id(1528).unwrap();
    let mut fire = block.data_as::<Ageable>().unwrap();
    assert_eq!(fire.age(), 1);
    fire.set_age(3);
    block.set_data(fire);
    assert_eq!(block.data_as::<Ageable>().unwrap().age(), 3);

    println!("{:?}", start.elapsed());
}

#[test]
fn set_only_valid_values() {
    let mut block = BlockState::from_id(1528).unwrap();
    let mut fire = block.data_as::<Ageable>().unwrap();
    assert_eq!(fire.age(), 1);
    fire.set_age(20);
    block.set_data(fire);
    fire = block.data_as::<Ageable>().unwrap();
    assert_eq!(fire.age(), 1);
    fire.set_age(15);
    block.set_data(fire);
    assert_eq!(block.data_as::<Ageable>().unwrap().age(), 15);
}

#[test]
fn block_data_valid_properties() {
    let block = BlockState::from_id(1528).unwrap();
    let fire = block.data_as::<Ageable>().unwrap();
    assert_eq!(
        fire.valid_age(),
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
    )
}

#[test]
fn block_state_valid_properties() {
    let block = BlockState::from_id(1528).unwrap();

    assert_eq!(
        block.get_valid_properties().age,
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
    );
    assert_eq!(block.get_valid_properties().up, vec![true, false]);
    assert_eq!(block.get_valid_properties().waterlogged, Vec::new())
}

#[test]
fn default_state() {
    let block = BlockState::new(BlockKind::PointedDripstone);
    assert_eq!(block.id(), 18_549);
    dbg!(block);
}

#[test]
fn stairs() {
    let block =
        BlockState::new(BlockKind::from_namespaced_id("minecraft:nether_brick_stairs").unwrap());
    assert_eq!(block.kind(), BlockKind::NetherBrickStairs);
}
