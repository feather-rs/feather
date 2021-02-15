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
