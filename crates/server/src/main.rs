use anyhow::Context;
use base::{Setup, TICK_MILLIS};
use config::Config;
use std::{
    sync::{atomic::AtomicUsize, Arc},
    thread::sleep,
    time::Duration,
    time::Instant,
};

mod config;
mod entity;
mod init;
mod network;
mod packet_handler;

/// Shared server state. Stored as a resource.
pub type Server = Arc<ServerInner>;

pub struct ServerInner {
    /// The number of online players.
    pub player_count: AtomicUsize,
    /// The server configuration.
    pub config: Config,
    /// The server icon, base64-encoded.
    pub icon: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let (mut state, executor) = init::init().context("failed to start server")?;

    loop {
        let start = Instant::now();
        executor.tick(&mut state);
        let elapsed = start.elapsed();

        match Duration::from_millis(TICK_MILLIS as u64).checked_sub(elapsed) {
            Some(remaining_time) => sleep(remaining_time),
            None => {
                log::debug!("Running behind! Tick took {:?}", elapsed);
                continue;
            }
        }
    }
}

pub fn setup(setup: &mut Setup) {
    entity::setup(setup);
    network::setup(setup);
}
