/*
This plugin observers the CreativeFlightEvent printing a msg when someone starts
flying.
*/

use quill::{events::CreativeFlyingEvent, Game, Plugin, Setup};

quill::plugin!(FlightPlugin);

struct FlightPlugin {}

impl Plugin for FlightPlugin {
    fn enable(_game: &mut Game, setup: &mut Setup<Self>) -> Self {
        setup.add_system(flight_observer_system);
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
