//! Feather, a Minecraft server implementation in Rust.
//!
//! For extensive developer documentation, please see [the book](https://feather-rs.github.io/book).

use feather_server_chunk::ChunkWorkerHandle;
use feather_server_lighting::LightingWorkerHandle;
use feather_server_types::{BanInfo, Game, ShutdownChannels, TPS};
use fecs::{Executor, OwnedResources, ResourcesProvider, World};
use spin_sleep::LoopHelper;
use std::ops::Deref;
use std::panic::AssertUnwindSafe;
use std::process::exit;
use std::sync::{Arc, RwLock};
use tokio::runtime;

mod event_handlers;
mod init;
mod shutdown;
mod systems;

struct FullState {
    resources: Arc<OwnedResources>,
    world: World,
    executor: Executor,
    shutdown_rx: crossbeam::Receiver<()>,
}

pub async fn main(runtime: runtime::Handle) {
    log::info!("Starting Feather; please wait");
    let (executor, resources, world) = match init::init(runtime).await {
        Ok(res) => res,
        Err(e) => {
            // Logging might not have been initialized yet - init it and ignore errors
            let _ = simple_logger::init();
            log::error!("Failed to start server: {:?}", e);
            log::error!("Exiting");
            exit(1);
        }
    };

    // Shutdown channels from wrapper resource are used to notify server thread of shutdown
    let (shutdown_tx, shutdown_rx) = {
        let channels = resources.get::<ShutdownChannels>();
        (channels.tx.clone(), channels.rx.clone())
    };
    shutdown::init(shutdown_tx);

    let state = FullState {
        resources,
        executor,
        world,
        shutdown_rx,
    };

    log::info!("Server started");
    let mut state = run_ticking_thread(state).await;

    log::info!("Shutting down");
    if let Err(e) = shut_down(&state.resources, &mut state.world).await {
        log::error!("An error occurred while shutting down: {:?}", e);
        log::error!("Exiting.");
        exit(1);
    }

    log::info!("Goodbye");
}

/// Starts the ticking thread. The returned future will complete
/// once the thread has terminated (i.e. the shutdown signal
/// has been received.)
async fn run_ticking_thread(mut state: FullState) -> FullState {
    use std::thread;
    use tokio::sync::oneshot;
    let (tx, rx) = oneshot::channel();

    thread::Builder::new()
        .name(String::from("feather"))
        .spawn(move || {
            match std::panic::catch_unwind(AssertUnwindSafe(|| {
                run_loop(&mut state);
            })) {
                Ok(_) => (),
                Err(_) => {
                    log::error!("The server crashed. This is a bug.");
                    log::error!(
                        "Please report this at https://github.com/feather-rs/feather/issues"
                    );
                }
            }

            tx.send(state).ok().expect("failed to exit server thread");
        })
        .expect("failed to spawn ticking thread");

    rx.await.unwrap()
}

/// Runs the main game loop.
fn run_loop(state: &mut FullState) {
    let mut loop_helper = LoopHelper::builder().build_with_target_rate(TPS as f64);
    loop {
        if state.shutdown_rx.try_recv().is_ok() {
            // Shut down
            return;
        }

        loop_helper.loop_start();

        // Execute all systems
        state
            .executor
            .execute(state.resources.deref(), &mut state.world);
        // Clean up world
        state.world.defrag(Some(256)); // should this be done at an interval rate?

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
    log::info!("Saving ban list");
    shutdown::save_ban_list(&resources.get::<Arc<RwLock<BanInfo>>>()).await?;
    log::info!("Waiting for tasks to finish");
    shutdown::wait_for_task_completion().await?;

    Ok(())
}
