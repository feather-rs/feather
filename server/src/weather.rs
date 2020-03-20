use crate::{network::Network, Game, World};
use feather_core::network::packet::implementation::ChangeGameState;
use fecs::Entity;
use rand::Rng;

const TICKS_DAY: i32 = 24_000;
const TICKS_HALF_DAY: i32 = TICKS_DAY / 2;
const TICKS_WEEK: i32 = TICKS_DAY * 7;
const THUNDER_FACTOR: i32 = 10;

#[derive(PartialEq)]
pub enum Weather {
    Clear,
    Rain,
    Thunder,
}

pub struct WeatherChangeEvent {
    pub from: Weather,
    pub to: Weather,
}

#[system]
pub fn handle_weather(game: &mut Game, world: &mut World) {
    let from = game.weather();

    if game.level.rain_time <= 0 {
        game.level.raining = !game.level.raining;

        let duration = if game.level.raining {
            game.rng().gen_range(TICKS_HALF_DAY, TICKS_DAY)
        } else {
            game.rng()
                .gen_range(TICKS_HALF_DAY, TICKS_WEEK + TICKS_HALF_DAY)
        };

        game.level.rain_time = duration;
    }

    if game.level.thunder_time <= 0 {
        game.level.thundering = !game.level.thundering;

        let duration = if game.level.raining {
            game.rng().gen_range(TICKS_HALF_DAY, TICKS_DAY)
        } else {
            // Let the time between thunderstorms be THUNDER_FACTOR larger than normal rain.
            // Ie. roughly 10 normal rain between thunderstorms.
            game.rng()
                .gen_range(TICKS_HALF_DAY, TICKS_WEEK + TICKS_HALF_DAY)
                * THUNDER_FACTOR
        };

        game.level.thunder_time = duration;
    }

    let to = game.weather();

    if from != to {
        game.on_weather_change(world, WeatherChangeEvent { from, to })
    }

    game.level.rain_time -= 1;
    game.level.thunder_time -= 1;
}

pub fn get_weather(game: &Game) -> Weather {
    match (game.level.raining, game.level.thundering) {
        (_, true) => Weather::Thunder,
        (true, false) => Weather::Rain,
        (false, false) => Weather::Clear,
    }
}

pub fn set_weather(world: &mut World, player: Entity, to: Weather) {
    let network = world.get::<Network>(player);

    network.send(create_weather_packet(to));
}

pub fn send_weather(game: &Game, world: &mut World, player: Entity) {
    let weather = game.weather();
    set_weather(world, player, weather)
}

pub fn broadcast_weather(game: &mut Game, world: &mut World, to: Weather) {
    game.broadcast_global(world, create_weather_packet(to), None);
}

fn create_weather_packet(to: Weather) -> ChangeGameState {
    let reason = match to {
        Weather::Rain | Weather::Thunder => 2,
        Weather::Clear => 1,
    };

    ChangeGameState {
        reason,
        value: 0f32,
    }
}
