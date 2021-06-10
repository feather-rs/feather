use std::{cell::RefCell, rc::Rc};

use anyhow::Context;
use common::{
    world_source::{flat::FlatWorldSource, region::RegionWorldSource, WorldSource},
    Game, TickLoop, World,
};
use ecs::SystemExecutor;
use feather_server::Server;
use plugin_host::PluginManager;

mod logging;

const PLUGINS_DIRECTORY: &str = "plugins";
const WORLD_DIRECTORY: &str = "world";
const CONFIG_PATH: &str = "config.toml";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let feather_server::config::ConfigContainer {
        config,
        was_config_created,
    } = feather_server::config::load(CONFIG_PATH).context("failed to load configuration file")?;
    logging::init(config.log.level);
    if was_config_created {
        log::info!("Created default config");
    }
    log::info!("Loaded config");

    log::info!("Creating server");
    let options = config.to_options();
    let server = Server::bind(options).await?;

    let game = init_game(server)?;

    run(game);

    Ok(())
}

fn init_game(server: Server) -> anyhow::Result<Game> {
    let mut game = Game::new();
    init_systems(&mut game, server);
    init_world_source(&mut game);
    init_plugin_manager(&mut game)?;
    Ok(game)
}

fn init_systems(game: &mut Game, server: Server) {
    let mut systems = SystemExecutor::new();

    // Register common before server code, so
    // that packet broadcasting happens after
    // gameplay actions.
    common::register(game, &mut systems);
    server.link_with_game(game, &mut systems);

    print_systems(&systems);

    game.system_executor = Rc::new(RefCell::new(systems));
}

fn init_world_source(game: &mut Game) {
    // Load chunks from the world save first,
    // and fall back to generating a superflat
    // world otherwise. This is a placeholder:
    // we don't have proper world generation yet.
    let world_source =
        RegionWorldSource::new(WORLD_DIRECTORY).with_fallback(FlatWorldSource::new());
    game.world = World::with_source(world_source);
}

fn init_plugin_manager(game: &mut Game) -> anyhow::Result<()> {
    let mut plugin_manager = PluginManager::new();
    plugin_manager.load_dir(game, PLUGINS_DIRECTORY)?;

    let plugin_manager_rc = Rc::new(RefCell::new(plugin_manager));
    game.insert_resource(plugin_manager_rc);
    Ok(())
}

fn print_systems(systems: &SystemExecutor<Game>) {
    let systems: Vec<&str> = systems.system_names().collect();
    log::debug!("---SYSTEMS---\n{:#?}\n", systems);
}

fn run(game: Game) {
    let tick_loop = create_tick_loop(game);
    log::debug!("Launching the game loop");
    tick_loop.run();
}

fn create_tick_loop(mut game: Game) -> TickLoop {
    TickLoop::new(move || {
        let systems = Rc::clone(&game.system_executor);
        systems.borrow_mut().run(&mut game);
        game.tick_count += 1;

        false
    })
}
