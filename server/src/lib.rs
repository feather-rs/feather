//! Feather, a Minecraft server implementation in Rust.
//!
//! For extensive developer documentation, please see [the book](https://feather-rs.github.io/book).

use feather_server_chunk::ChunkWorkerHandle;
use feather_server_lighting::LightingWorkerHandle;
use feather_server_types::{Game, TPS};
use fecs::{Executor, OwnedResources, ResourcesProvider, World};
use spin_sleep::LoopHelper;
use std::process::exit;

mod event_handlers;
mod init;
mod shutdown;
mod systems;

pub async fn main() {
    log::info!("Starting Feather; please wait");
    let (executor, resources, mut world) = match init::init().await {
        Ok(res) => res,
        Err(e) => {
            // Logging might not have been initialized yet - init it and ignore errors
            let _ = simple_logger::init();
            log::error!("Failed to start server: {}", e);
            log::error!("Exiting");
            exit(1);
        }
    };

    // Channels used by the shutdown handler thread
    // to notify server thread of shutdown
    let (shutdown_tx, shutdown_rx) = crossbeam::bounded(1);
    shutdown::init(shutdown_tx);

    log::info!("Server started");
    run_loop(&mut world, &resources, &executor, shutdown_rx);

    log::info!("Shutting down");
    shut_down(&resources, &mut world).await;

    log::info!("Goodbye");
    exit(0);
}

/// Runs the main game loop.
fn run_loop(
    world: &mut World,
    resources: &OwnedResources,
    executor: &Executor,
    shutdown_rx: crossbeam::Receiver<()>,
) {
    let mut loop_helper = LoopHelper::builder().build_with_target_rate(TPS as f64);
    loop {
        if shutdown_rx.try_recv().is_ok() {
            // Shut down
            return;
        }

        loop_helper.loop_start();

        // Execute all systems
        executor.execute(resources, world);
        // Clean up world
        world.defrag(Some(256)); // should this be done at an interval rate?

        loop_helper.loop_sleep();
    }
}

async fn shut_down(resources: &OwnedResources, world: &mut World) -> anyhow::Result<()> {
    log::info!("Disconnecting players");
    shutdown::disconnect_players(&world)?;
    log::info!("Shutting down workers");
    shutdown::shut_down_workers(
        &*resources.get::<Game>(),
        &*resources.get::<LightingWorkerHandle>(),
    )?;
    log::info!("Saving chunks");
    shutdown::save_chunks(
        &*resources.get::<Game>(),
        &*resources.get::<ChunkWorkerHandle>(),
        &world,
    )?;
    log::info!("Saving level.dat");
    shutdown::save_level(&mut *resources.get_mut::<Game>()).await?;
    log::info!("Saving player data");
    shutdown::save_player_data(&*resources.get::<Game>(), &world)?;
    log::info!("Waiting for tasks to finish");
    shutdown::wait_for_task_completion(&*resources.get::<Game>()).await?;

    Ok(())
}
