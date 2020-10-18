//! Startup logic.

use crate::{event_handlers, systems};
use anyhow::Context;
use feather_core::anvil::level::{LevelData, LevelGeneratorType};
use feather_core::util::ChunkPosition;
use feather_server_chunk::{chunk_worker, ChunkWorkerHandle};
use feather_server_config::DEFAULT_CONFIG_STR;
use feather_server_network::NetworkIoManager;
use feather_server_packet_buffer::PacketBuffers;
use feather_server_types::{task, BanInfo, Config, Game, Shared, ShutdownChannels};
use feather_server_worldgen::{
    ComposableGenerator, EmptyWorldGenerator, SuperflatWorldGenerator, WorldGenerator,
};
use fecs::{EntityBuilder, Executor, OwnedResources, ResourcesProvider, World};
use fxhash::FxHasher;
use rand::Rng;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::{io, runtime};

/// Intializes the server.
pub async fn init(
    runtime: runtime::Handle,
) -> anyhow::Result<(Executor, Arc<OwnedResources>, World)> {
    let mut executor = systems::build_executor();
    let mut event_handlers = event_handlers::build_event_handlers();

    let mut world = World::new();
    let mut resources = OwnedResources::new();
    executor.set_up(&mut resources, &mut world);
    event_handlers.set_up(&mut resources, &mut world);

    println!("Loading configuration");
    let config = load_config()
        .await
        .context("Failed to load configuration file `feather.toml`")?;
    set_up_logging(&config).context("Failed to initialize logging")?;

    log::info!("Loading ban list");
    let ban_info = load_ban_info()
        .await
        .context("Failed to load ban list `bans.toml`")?;

    log::info!("Loading world save");
    let level = load_level(&config)
        .await
        .context("Failed to load level file (is your world directory corrupted?)")?;

    let cworker_handle = create_cworker_handle(&config, &level);

    let mut game = Game {
        shared: Arc::new(Shared {
            config: Arc::clone(&config),
            rng: Default::default(),
            player_count: Arc::new(Default::default()),
        }),
        chunk_map: Default::default(),
        tick_count: 0,
        chunk_holders: Default::default(),
        block_entities: Default::default(),
        level,
        chunk_entities: Default::default(),
        time: Default::default(),
        event_handlers: Arc::new(event_handlers),
        resources: Arc::new(Default::default()), // we override this momentarily
        bump: Default::default(),
        game_rules: Default::default(),
    };
    task::init(runtime);
    let packet_buffers = Arc::new(PacketBuffers::new());

    log::info!("Queueing spawn chunks for loading");
    load_spawn_chunks(&mut game, &mut world, &cworker_handle);

    log::info!("Creating RSA keypair");
    feather_server_network::init();

    log::info!("Initializing block ID mappings");
    feather_core::blocks::init();

    log::info!("Starting networking task");
    let networking_handle = create_networking_handle(
        Arc::clone(&config),
        Arc::clone(&ban_info),
        &game,
        Arc::clone(&packet_buffers),
    )
    .await
    .context("Failed to start the networking task")?;

    let resources = create_resources(
        resources,
        game,
        cworker_handle,
        networking_handle,
        packet_buffers,
        ban_info,
    );

    Ok((executor, resources, world))
}

async fn load_config() -> anyhow::Result<Arc<Config>> {
    const PATH: &str = "feather.toml";

    match File::open(PATH).await {
        Ok(mut file) => Config::load_from_file(&mut file).await,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            println!("Missing configuration file; creating a default one for you.");

            let mut file = File::create(PATH).await?;
            file.write_all(DEFAULT_CONFIG_STR.as_bytes()).await?;

            let config = Config::default();
            Ok(config)
        }
        Err(e) => Err(e.into()),
    }
    .map(Arc::new)
}

async fn load_ban_info() -> anyhow::Result<Arc<RwLock<BanInfo>>> {
    const PATH: &str = "bans.toml";

    match File::open(PATH).await {
        Ok(mut file) => BanInfo::load_from_file(&mut file).await,
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(BanInfo::default()),
        Err(e) => Err(e.into()),
    }
    .map(RwLock::new)
    .map(Arc::new)
}

fn set_up_logging(config: &Config) -> anyhow::Result<()> {
    use log::Level::*;
    let level = match config.log.level.as_str() {
        "error" => Error,
        "warn" => Warn,
        "info" => Info,
        "debug" => Debug,
        "trace" => Trace,
        x => anyhow::bail!(
            "invalid logging level {} (please check your config file)",
            x
        ),
    };

    simple_logger::init_with_level(level).map_err(Into::into)
}

async fn load_level(config: &Config) -> anyhow::Result<LevelData> {
    const LEVEL_FILE_NAME: &str = "level.dat";
    let world_dir = Path::new(&config.world.name);

    // Create world directory (silently fail if it already exists)
    let _ = tokio::fs::create_dir(world_dir).await;

    let mut level_path = PathBuf::new();
    level_path.push(world_dir);
    level_path.push(LEVEL_FILE_NAME);

    match File::open(&level_path).await {
        Ok(mut file) => LevelData::load_from_file(&mut file).await,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            log::info!("World save not found; creating it");
            let level = generate_level(config);
            let mut file = File::create(&level_path).await?;
            level.save_to_file(&mut file).await?;

            Ok(level)
        }
        Err(e) => Err(e.into()),
    }
}

fn generate_level(config: &Config) -> LevelData {
    let seed = seed_for_config(config);
    let world_name = &config.world.name;
    log::info!("Using seed {} for world '{}'", seed, world_name);

    // TODO: Generate spawn position properly
    LevelData {
        allow_commands: false,
        border_center_x: 0.0,
        border_center_z: 0.0,
        border_damage_per_block: 0.0,
        border_safe_zone: 0.0,
        border_size: 0.0,
        clear_weather_time: 0,
        data_version: 0,
        day_time: 0,
        difficulty: 0,
        difficulty_locked: 0,
        game_type: 0,
        hardcore: false,
        initialized: false,
        last_played: 0,
        raining: false,
        rain_time: 0,
        seed,
        spawn_x: 0,
        spawn_y: 100,
        spawn_z: 0,
        thundering: false,
        thunder_time: 0,
        time: 0,
        version: Default::default(),
        generator_name: config.world.generator.to_string(),
        generator_options: None,
    }
}

fn seed_for_config(config: &Config) -> i64 {
    let seed_raw = &config.world.seed;
    // Empty seed: random
    // Seed is valid i64: parse
    // Seed is something else: hash
    if seed_raw.is_empty() {
        rand::thread_rng().gen()
    } else {
        match seed_raw.parse::<i64>() {
            Ok(seed_int) => seed_int,
            Err(_) => hash_seed(seed_raw.as_str()),
        }
    }
}

fn hash_seed(seed_raw: &str) -> i64 {
    // use FxHash instead of DefaultHasher because
    // it's deterministic
    let mut hasher = FxHasher::default();
    seed_raw.hash(&mut hasher);
    hasher.finish() as i64
}

fn create_cworker_handle(config: &Config, level: &LevelData) -> ChunkWorkerHandle {
    let generator: Arc<dyn WorldGenerator> = match level.generator_type() {
        LevelGeneratorType::Flat => Arc::new(SuperflatWorldGenerator {
            options: level.clone().generator_options.unwrap_or_default(),
        }),
        LevelGeneratorType::Default => {
            Arc::new(ComposableGenerator::default_with_seed(level.seed as u64))
        }
        _ => Arc::new(EmptyWorldGenerator {}),
    };

    let (tx, rx) = chunk_worker::start(Path::new(&config.world.name), generator);
    ChunkWorkerHandle {
        sender: tx,
        receiver: rx,
    }
}

async fn create_networking_handle(
    config: Arc<Config>,
    ban_info: Arc<RwLock<BanInfo>>,
    game: &Game,
    packet_buffers: Arc<PacketBuffers>,
) -> anyhow::Result<NetworkIoManager> {
    let server_icon = load_server_icon()
        .await
        .context("failed to load server icon `server-icon.png` (is it corrupted?)")?;

    let addr = format!("{}:{}", config.server.address, config.server.port);
    let socket = TcpListener::bind(&addr)
        .await
        .context("failed to bind to port (is another server instance already running?)")?;

    log::info!("Listening on {}", addr);

    Ok(NetworkIoManager::start(
        socket,
        config,
        ban_info,
        Arc::clone(&game.player_count),
        Arc::new(server_icon),
        packet_buffers,
    ))
}

async fn load_server_icon() -> anyhow::Result<Option<String>> {
    match File::open("server-icon.png").await {
        Ok(mut file) => {
            let mut buf = vec![];
            file.read_to_end(&mut buf).await?;

            let encoded = base64::encode(&buf);
            Ok(Some(format!("data:image/png;base64,{}", encoded)))
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.into()),
    }
}

/// Loads the chunks around the spawn area and creates
/// a chunk hold on those chunks to prevent them from
/// being unloaded.
///
/// Note that these chunks are loaded asynchronously,
/// and this function will return before loading is complete.
fn load_spawn_chunks(game: &mut Game, world: &mut World, cworker_handle: &ChunkWorkerHandle) {
    let view_distance = i32::from(game.config.server.view_distance);

    // Create an entity for the server and
    // add chunk holders using it.
    let server_entity = EntityBuilder::new().build().spawn_in(world);

    let offset_x = game.level.spawn_x / 16;
    let offset_z = game.level.spawn_z / 16;
    for x in -view_distance..=view_distance {
        for z in -view_distance..=view_distance {
            let chunk = ChunkPosition::new(x + offset_x, z + offset_z);

            feather_server_chunk::load_chunk(cworker_handle, chunk);
            game.chunk_holders.insert_holder(chunk, server_entity);
        }
    }
}

fn create_resources(
    resources: OwnedResources,
    game: Game,
    cworker_handle: ChunkWorkerHandle,
    networking_handle: NetworkIoManager,
    packet_buffers: Arc<PacketBuffers>,
    ban_info: Arc<RwLock<BanInfo>>,
) -> Arc<OwnedResources> {
    let resources = {
        let resources = resources
            .with(game)
            .with(cworker_handle)
            .with(networking_handle)
            .with(packet_buffers)
            .with(ban_info)
            .with(ShutdownChannels::new());
        Arc::new(resources)
    };

    resources.get_mut::<Game>().resources = Arc::clone(&resources);

    resources
}
