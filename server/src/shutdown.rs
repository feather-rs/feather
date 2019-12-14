//! Shutdown behavior.
use crossbeam::Sender;
use legion::world::World;

pub fn init(tx: Sender<()>) {
    ctrlc::set_handler(move || {
        tx.send(()).unwrap();
    })
    .unwrap();
}

pub fn save_chunks(world: &mut World) {
    unimplemented!()
}

pub fn save_level(world: &World) {
    unimplemented!()
}

pub fn save_player_data(world: &World) {
    unimplemented!()
}
