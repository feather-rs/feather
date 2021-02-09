#![no_main]

use quill::{Game, Plugin, Position, Setup};

quill::plugin!(SimplePlugin);

struct SimplePlugin;

impl Plugin for SimplePlugin {
    fn enable(_game: &mut Game, setup: &mut Setup<Self>) -> Self {
        setup.add_system(test_system);
        SimplePlugin
    }

    fn disable(self, _game: &mut Game) {}
}

fn test_system(_plugin: &mut SimplePlugin, game: &mut Game) {
    for (entity, position) in game.query::<&Position>() {
        entity.send_message(format!("Your position is {:.1?}", position));
    }
}
