use std::{cell::RefCell, rc::Rc, sync::Arc};

use anyhow::{anyhow, bail, Context};
use quill::world::{WorldDescriptor, WorldSettings};
use quill::{Game as _, WorldId};
use tokio::runtime::Runtime;

use crate::{config::Config, logging, Server};
use common::{Game, TickLoop};
use libcraft::biome::{ BiomeList};
use vane::SystemExecutor;

const CONFIG_PATH: &str = "config.toml";

pub async fn create_game(runtime: Runtime) -> anyhow::Result<Game> {
    let crate::config::ConfigContainer {
        config,
        was_config_created,
    } = crate::config::load(CONFIG_PATH).context("failed to load configuration file")?;
    logging::init(config.log.level);
    if was_config_created {
        log::info!("Created default config");
    }
    log::info!("Loaded config");

    log::info!("Creating server");
    let options = config.to_options();
    let server = Server::bind(options).await?;

    let mut game = init_game(server, &config, runtime)?;
    game.insert_resource(config);

    Ok(game)
}

pub fn run(game: Game) -> anyhow::Result<()> {
    launch(game)
}

fn init_game(server: Server, _config: &Config, runtime: Runtime) -> anyhow::Result<Game> {
    let mut game = Game::new(runtime);
    init_systems(&mut game, server);
    init_biomes(&mut game);
    Ok(game)
}

fn init_systems(game: &mut Game, server: Server) {
    let mut systems = SystemExecutor::new();

    // Register common before server code, so
    // that packet broadcasting happens after
    // gameplay actions.
    common::register(game, &mut systems);
    server.link_with_game(game, &mut systems);

    game.system_executor = Rc::new(RefCell::new(systems));
}

fn init_biomes(game: &mut Game) {
    let biomes = Arc::new(BiomeList::vanilla());
    game.insert_resource(biomes);
}

fn launch(mut game: Game) -> anyhow::Result<()> {
    // World initialization must happen after plugin initialization
    // so plugin world sources can be referenced in the `config.toml`.
    init_worlds(&mut game)?;

    let tick_loop = create_tick_loop(game);
    log::debug!("Launching the game loop");
    tick_loop.run();
    Ok(())
}

fn init_worlds(game: &mut Game) -> anyhow::Result<()> {
    let config = game.resources.get::<Config>()?.clone();
    let dimension_types = libcraft::dimension::vanilla_dimensions();
    let mut default_world_set = false;
    for (world_name, world) in config.worlds {
        let dimension_info = dimension_types
            .iter()
            .find(|dim| dim.r#type == world.dimension_type)
            .ok_or_else(|| {
                anyhow!(
                    "world '{}' has unknown dimension type '{}'",
                    world_name,
                    world.dimension_type
                )
            })?
            .clone();

        let source_factory = game
            .world_source_factory(&world.source.typ)
            .with_context(|| format!("unknown world source in world '{}'", world_name))?;
        let source = source_factory
            .create_world_source(
                game,
                &toml::Value::Table(world.source.params),
                &dimension_info,
            )
            .with_context(|| {
                format!(
                    "failed to initialize world source for world '{}'",
                    world_name
                )
            })?;

        let id = WorldId::new_random(); // TODO persist
        let desc = WorldDescriptor {
            id,
            source,
            name: Some(world_name.clone()),
            dimension_info,
            flat: world.flat,
            settings: WorldSettings {
                save_strategy: world.save_strategy.into(),
                ..Default::default()
            },
        };
        game.create_world(desc);

        if world_name == config.server.default_world {
            game.set_default_world(id);
            default_world_set = true;
        }
    }

    if !default_world_set {
        bail!(
            "default world '{}' is not configured",
            config.server.default_world
        );
    }

    Ok(())
}

fn create_tick_loop(mut game: Game) -> TickLoop {
    TickLoop::new(move || {
        let systems = Rc::clone(&game.system_executor);
        systems.borrow_mut().run(&mut game);
        game.tick_count += 1;

        false
    })
}
