use std::time::Instant;

use libcraft_blocks::{Ageable, BlockState};

#[test]
fn update_block_data() {
    let start = Instant::now();

    let mut block = BlockState::from_id(1485).unwrap();
    let mut fire = block.data_as::<Ageable>().unwrap();
    assert_eq!(fire.age, 1);
    fire.age = 3;
    block.set_data(fire);
    assert_eq!(block.data_as::<Ageable>().unwrap().age, 3);

    println!("{:?}", start.elapsed());
}

#[test]
fn correct_valid_properties() {
    let block = BlockState::from_id(1485).unwrap();

    assert_eq!(
        block.get_valid_properties().age,
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
    );
    assert_eq!(block.get_valid_properties().up, vec![true, false]);
    assert_eq!(block.get_valid_properties().waterlogged, Vec::new())
}
