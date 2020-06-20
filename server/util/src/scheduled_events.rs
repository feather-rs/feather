use feather_server_types::Game;
use fecs::World;

#[fecs::system]
pub fn handle_scheduled_events(game: &mut Game, world: &mut World) {
    if game.event_scheduler.hash_elements(game.tick_count) {
        for event in game.event_scheduler.poll(game.tick_count) {
            println!("Handle event: {:?}", event);
            game.handle(world, event);
        }
    }
}