use std::{cell::RefCell, rc::Rc, sync::Arc};

use base::anvil::level::SuperflatGeneratorOptions;
use common::{Game, TickLoop, World};
use ecs::{HasResources, SystemExecutor};
use feather_commands::{CommandCtx, CommandDispatcher};
use feather_server::console_input::{flush_stdout, ConsoleInput};
use feather_server::{config::Config, Server};
use plugin_host::PluginManager;
use quill_common::components::DefaultGamemode;
use worldgen::{ComposableGenerator, SuperflatWorldGenerator, WorldGenerator};

mod logging;

const PLUGINS_DIRECTORY: &str = "plugins";
const CONFIG_PATH: &str = "config.toml";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let feather_server::config::ConfigContainer {
        config,
        was_config_created,
    } = feather_server::config::load(CONFIG_PATH).expect("failed to load configuration file");

    let (stdout_tx, stdout_rx) = flume::unbounded();
    logging::init(config.log.level, stdout_tx);
    if was_config_created {
        log::info!("Created default config");
    }
    log::info!("Loaded config");

    log::info!("Creating server");
    flush_stdout(&stdout_rx, "");
    let options = config.to_options();
    match Server::bind(options).await {
        Ok(server) => match init_game(server, &config) {
            Ok(mut game) => {
                let console_input = ConsoleInput::new::<CommandCtx>(
                    stdout_rx,
                    config.cli.completion_type,
                    config.cli.edit_mode,
                );
                game.insert_resource(console_input);
                run(game);
            }
            Err(err) => {
                log::error!("{}", err);
                flush_stdout(&stdout_rx, "");
                std::process::exit(1);
            }
        },
        Err(err) => {
            log::error!("{}", err);
            flush_stdout(&stdout_rx, "");
            std::process::exit(1);
        }
    }

    Ok(())
}

fn init_game(server: Server, config: &Config) -> anyhow::Result<Game> {
    let mut game = Game::new();
    init_systems(&mut game, server);
    init_world_source(&mut game, config);
    init_commands(&mut game);
    init_plugin_manager(&mut game)?;
    init_config(&mut game, config);
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

    game.insert_resource(systems);
}

fn init_world_source(game: &mut Game, config: &Config) {
    // Load chunks from the world save first,
    // and fall back to generating a world otherwise.

    // FIXME: load from the level file if exists
    let seed = config.world.seed.parse().unwrap_or_else(|_| {
        if config.world.seed.is_empty() {
            rand::random()
        } else {
            // Java String#hashCode
            let mut hashcode = 0i32;
            for c in config.world.seed.chars() {
                hashcode = hashcode.overflowing_mul(31).0 + c as i32;
            }
            hashcode as u64
        }
    });

    let generator: Arc<dyn WorldGenerator> = match &config.world.generator[..] {
        "flat" => Arc::new(SuperflatWorldGenerator::new(
            SuperflatGeneratorOptions::default(),
        )),
        _ => Arc::new(ComposableGenerator::default_with_seed(seed)),
    };
    game.world = World::new(generator, config.world.name.clone(), seed);
}

fn init_commands(game: &mut Game) {
    let mut dispatcher = CommandDispatcher::new();
    feather_commands::register_vanilla_commands(&mut dispatcher);
    game.insert_resource(dispatcher);
}

fn init_plugin_manager(game: &mut Game) -> anyhow::Result<()> {
    let mut plugin_manager = PluginManager::new();
    plugin_manager.load_dir(game, PLUGINS_DIRECTORY)?;

    let plugin_manager_rc = Rc::new(RefCell::new(plugin_manager));
    game.insert_resource(plugin_manager_rc);
    Ok(())
}

fn init_config(game: &mut Game, config: &Config) {
    game.insert_resource(DefaultGamemode::new(config.server.default_gamemode));
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
        game.resources()
            .get_mut::<SystemExecutor<Game>>()
            .unwrap()
            .run(&mut game);
        game.tick_count += 1;

        false
    })
}
