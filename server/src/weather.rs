use crate::Game;
use rand::Rng;


enum Weather {
    Clear,
    Rain,
    Thunder
}

#[system]
fn handle_weather(game: &mut Game) {
    game.level.rain_time -= 1;
    game.level.thunder_time -= 1;

    let from = weather(game);

    if game.level.rain_time <= 0 {

        game.level.raining = !game.level.raining;
    }

    if game.level.thunder_time <= 0 {

        game.level.thundering = !game.level.thundering;
    }

    let to = weather(game);
}

fn weather(game: &Game) -> Weather {
    match (game.level.raining, game.level.thundering) {
        (_, true) => Weather::Thunder,
        (true, false) => Weather::Rain,
        (false, false) => Weather::Clear,
    }
}