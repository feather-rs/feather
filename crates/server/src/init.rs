use std::{
    fs, fs::File, net::SocketAddr, path::Path, sync::atomic::AtomicUsize, sync::Arc, thread,
};

use anyhow::Context;
use async_executor::Executor;
use base::{Setup, State};
use ecs::SystemExecutor;
use future::block_on;
use futures_lite::future;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use utils::{BlockingPool, ComputePool};

use crate::{config::Config, network::Listener, network::ListenerHandle, Server, ServerInner};

/// Initializes the server, performing all startup tasks.
pub fn init() -> anyhow::Result<(State, SystemExecutor<State>)> {
    let config = load_config()?;
    init_logging(&config);
    log::info!("Starting up");

    let icon = load_icon()?;
    let compute_pool = ComputePool::new(8);
    let blocking_pool = BlockingPool::new();
    let server = Arc::new(ServerInner {
        player_count: AtomicUsize::new(0),
        config,
        icon,
        compute_pool,
        blocking_pool,
    });

    let executor = init_executor();
    let listener = init_listener(&server, &executor)?;
    log::info!(
        "Listening on {}:{}",
        server.config.server.address,
        server.config.server.port
    );

    let mut setup = Setup::new();
    setup.resource(server).resource(listener).resource(executor);

    common::setup(&mut setup);
    crate::setup(&mut setup);

    Ok(setup.build())
}

const CONFIG_NAME: &str = "config.toml";

fn load_config() -> anyhow::Result<Config> {
    if !Path::new(CONFIG_NAME).exists() {
        println!("Config not found. Creating it");
        Config::default().save_to_file(&mut File::create(CONFIG_NAME)?)?;
        Ok(Config::default())
    } else {
        Config::load_from_file(&mut File::open(CONFIG_NAME)?)
            .context("failed to parse configuration")
    }
}

fn init_logging(config: &Config) {
    let level = match config.log.level.as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        level => {
            println!("Unknown log level '{}'; defaulting to 'info'", level);
            LevelFilter::Info
        }
    };
    SimpleLogger::new().with_level(level).init().unwrap();
}

const ICON_NAME: &str = "server-icon.png";

fn load_icon() -> anyhow::Result<Option<String>> {
    if Path::new(ICON_NAME).exists() {
        let bytes = fs::read(ICON_NAME)?;
        let string = base64::encode(&bytes);
        Ok(Some(format!("data:image/png;base64,{}", string)))
    } else {
        Ok(None)
    }
}

fn init_listener(server: &Server, executor: &Arc<Executor>) -> anyhow::Result<ListenerHandle> {
    let addr = SocketAddr::new(
        server.config.server.address.parse()?,
        server.config.server.port,
    );

    let listener = Listener::new(addr, &executor, server)
        .with_context(|| format!("failed to bind to {}. Is a server already running?", addr))?;
    let handle = listener.handle();
    executor
        .spawn(async move {
            listener.run().await;
        })
        .detach();
    Ok(handle)
}

fn init_executor() -> Arc<Executor> {
    let executor = Arc::new(Executor::new());

    for i in 0..8 {
        let executor = Arc::clone(&executor);
        thread::Builder::new()
            .name(format!("async thread #{}", i))
            .spawn(move || {
                block_on(executor.run(future::pending::<()>()));
            })
            .expect("failed to create thread");
    }

    executor
}
