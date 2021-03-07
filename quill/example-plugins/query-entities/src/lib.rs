//! An example plugin that spawns 10,000 entities
//! on startup, then moves them each tick using a query.

use quill::{EntityInit, Game, Plugin, Position, entities::{PiglinBrute, Player}, send_message::SendMessage};
use rand::Rng;

quill::plugin!(QueryEntities);

struct QueryEntities {
    tick_counter: u64,
}

impl Plugin for QueryEntities {
    fn enable(game: &mut quill::Game, setup: &mut quill::Setup<Self>) -> Self {
        // Spawn 10,000 piglin brutes
        for x in 0..100 {
            for z in 0..100 {
                let pos = Position {
                    x: (x - 50) as f64 * 12.0,
                    y: 64.0,
                    z: (z - 50) as f64 * 12.0,
                    pitch: rand::thread_rng().gen_range(30.0..330.0),
                    yaw: rand::thread_rng().gen_range(0.0..360.0),
                };
                game.create_entity_builder(pos, EntityInit::PiglinBrute)
                    .finish();
            }
        }

        setup.add_system(query_system);

        Self { tick_counter: 0 }
    }

    fn disable(self, _game: &mut quill::Game) {}
}

fn query_system(plugin: &mut QueryEntities, game: &mut Game) {
    // Make the piglin brutes float into the air.
    plugin.tick_counter += 1;
    for (entity, (position, _piglin_brute)) in game.query::<(&Position, &PiglinBrute)>() {
        // Mutable access to components through queries
        // is not yet implemented, so we have to set
        // the component directly.
        entity.set(Position {
            y: position.y + 0.1 * ((plugin.tick_counter as f64 / 20.0).sin() + 1.0),
            ..position
        });
    }
 
    for (entity, (player,)) in game.query::<(&Player,)>() {
        (&entity, (&player,)).send_message("Hello world!");
        (&entity, (&player,)).send_message("Hello world!");        
    }
}
