use std::path::PathBuf;
use std::{cell::RefCell, rc::Rc, sync::Arc};

use anyhow::{anyhow, bail, Context};
use libcraft::dimension::DimensionInfo;
use quill::world::{WorldDescriptor, WorldSettings};
use quill::{Game as _, WorldId};
use tokio::runtime::Runtime;

use crate::{config::Config, logging, Server};
use common::{Game, TickLoop};
use data_generators::extract_vanilla_data;
use libcraft::biome::{BiomeGeneratorInfo, BiomeList};
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

    extract_vanilla_data();

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

fn init_game(server: Server, config: &Config, runtime: Runtime) -> anyhow::Result<Game> {
    let mut game = Game::new(runtime);
    init_systems(&mut game, server);
    init_biomes(&mut game)?;
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

fn init_biomes(game: &mut Game) -> anyhow::Result<()> {
    let mut biomes = BiomeList::default();

    let worldgen = PathBuf::from("worldgen");
    for dir in std::fs::read_dir(&worldgen)
        .context("There's no worldgen/ directory. Try removing generated/ and re-running feather")?
        .flatten()
    {
        let namespace = dir
            .file_name()
            .to_str()
            .context(format!(
                "Non-UTF8 characters in namespace directory: {:?}",
                dir.file_name()
            ))?
            .to_string();
        let namespace_dir = dir.path();
        let namespace_worldgen = namespace_dir.join("worldgen");
        for file in std::fs::read_dir(namespace_worldgen.join("biome")).context(
            format!("There's no worldgen/{}/worldgen/biome/ directory. Try removing generated/ and re-running feather",
                    dir.file_name().to_str().unwrap_or("<non-UTF8 characters>")),
        )?.flatten() {
            if let Some(file_name) = file.file_name().to_str() {
                if file_name.ends_with(".json") {
                    let biome: BiomeGeneratorInfo = serde_json::from_str(
                        &std::fs::read_to_string(file.path()).unwrap(),
                    )
                    .unwrap();
                    let name = format!(
                        "{}:{}",
                        namespace,
                        file_name.strip_suffix(".json").unwrap()
                    );
                    log::trace!("Loaded biome: {}", name);
                    biomes.insert(name, biome);
                }
            } else {
                // non-utf8 namespaces are errors, but non-utf8 values are just ignored
                log::warn!("Ignoring a biome file with non-UTF8 characters in name: {:?}", file.file_name())
            }
        }
    }
    game.insert_resource(Arc::new(biomes));
    Ok(())
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
    let dimension_types = load_dimension_types()?;
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

fn load_dimension_types() -> anyhow::Result<Vec<DimensionInfo>> {
    let mut types = Vec::new();
    let worldgen = PathBuf::from("worldgen");
    for namespace in std::fs::read_dir(&worldgen)
        .context("There's no worldgen/ directory. Try removing generated/ and re-running feather")?
        .flatten()
    {
        let namespace_path = namespace.path();
        for file in std::fs::read_dir(namespace_path.join("dimension"))?.flatten() {
            if file.path().is_dir() {
                bail!(
                    "worldgen/{}/dimension/ shouldn't contain directories",
                    file.file_name().to_str().unwrap_or("<non-UTF8 characters>")
                )
            }
            let mut dimension_info: DimensionInfo =
                serde_json::from_str(&std::fs::read_to_string(file.path()).unwrap())
                    .context("Invalid dimension format")?;

            let (dimension_namespace, dimension_value) =
                dimension_info.r#type.split_once(':').context(format!(
                    "Invalid dimension type `{}`. It should contain `:` once",
                    dimension_info.r#type
                ))?;
            if dimension_value.contains(':') {
                bail!(
                    "Invalid dimension type `{}`. It should contain `:` exactly once",
                    dimension_info.r#type
                );
            }
            let mut dimension_type_path = worldgen.join(dimension_namespace);
            dimension_type_path.push("dimension_type");
            dimension_type_path.push(format!("{}.json", dimension_value));
            dimension_info.info =
                serde_json::from_str(&std::fs::read_to_string(dimension_type_path).unwrap())
                    .context(format!(
                        "Invalid dimension type format (worldgen/{}/dimension_type/{}.json",
                        dimension_namespace, dimension_value
                    ))?;
            types.push(dimension_info);
        }
    }
    Ok(types)
}

fn create_tick_loop(mut game: Game) -> TickLoop {
    TickLoop::new(move || {
        let systems = Rc::clone(&game.system_executor);
        systems.borrow_mut().run(&mut game);
        game.tick_count += 1;

        false
    })
}
