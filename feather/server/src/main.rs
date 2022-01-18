use std::{cell::RefCell, rc::Rc, sync::Arc};

use anyhow::Context;
use base::anvil::level::SuperflatGeneratorOptions;
use common::{Game, TickLoop, World};
use ecs::SystemExecutor;
use feather_server::{config::Config, Server};
use plugin_host::PluginManager;
use worldgen::{ComposableGenerator, SuperflatWorldGenerator, VoidWorldGenerator, WorldGenerator};

mod logging;

const PLUGINS_DIRECTORY: &str = "plugins";
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

    let game = init_game(server, &config)?;

    run(game);

    Ok(())
}

fn init_game(server: Server, config: &Config) -> anyhow::Result<Game> {
    let mut game = Game::new();
    init_systems(&mut game, server);
    init_world_source(&mut game, config);
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

fn init_world_source(game: &mut Game, config: &Config) {
    // Load chunks from the world save first,
    // and fall back to generating a superflat
    // world otherwise. This is a placeholder:
    // we don't have proper world generation yet.

    let seed = 42; // FIXME: load from the level file

    let generator: Arc<dyn WorldGenerator> = match &config.world.generator[..] {
        "flat" => Arc::new(SuperflatWorldGenerator::new(
            SuperflatGeneratorOptions::default(),
        )),
        "void" => Arc::new(VoidWorldGenerator),
        _ => Arc::new(ComposableGenerator::default_with_seed(seed)),
    };
    game.world = World::with_gen_and_path(generator, config.world.name.clone());
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
