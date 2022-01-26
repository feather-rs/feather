/*
This plugin observers the CreativeFlightEvent printing a msg when someone starts
flying.
*/

use quill::{
    components::Sprinting,
    events::{CreativeFlyingEvent, SneakEvent},
    Game, Plugin, Setup,
};

#[quill::plugin]
struct FlightPlugin {}

impl Plugin for FlightPlugin {
    fn enable(_game: &mut Game, setup: &mut Setup<Self>) -> Self {
        setup.add_system(flight_observer_system);
        setup.add_system(sneak_observer_system);
        setup.add_system(sprinting_observer_system);
        FlightPlugin {}
    }

    fn disable(self, _game: &mut Game) {}
}

fn flight_observer_system(_plugin: &mut FlightPlugin, game: &mut Game) {
    for (entity, change) in game.query::<&CreativeFlyingEvent>() {
        if change.is_flying {
            entity.send_message("Enjoy your flight!");
        } else {
            entity.send_message("Hope you enjoyed your flight.");
        }
    }
}

fn sneak_observer_system(_plugin: &mut FlightPlugin, game: &mut Game) {
    for (player, change) in game.query::<&SneakEvent>() {
        if change.is_sneaking {
            player.send_message("Enjoy sneaking!");
        } else {
            player.send_message("How was it to be sneaking?");
        }
    }
}

fn sprinting_observer_system(_plugin: &mut FlightPlugin, game: &mut Game) {
    for (player, sprinting) in game.query::<&Sprinting>() {
        if sprinting.0 {
            player.send_message("Are you sprinting?");
        }
    }
}
