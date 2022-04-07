use common::Game;
use quill::Plugin;
use tokio::runtime::Runtime;
use vane::SystemExecutor;

pub struct ServerBuilder {
    game: Game,
}

impl ServerBuilder {
    pub fn new() -> anyhow::Result<Self> {
        let runtime = build_tokio_runtime();
        let handle = runtime.handle().clone();
        let game = handle.block_on(async move { crate::init::create_game(runtime).await })?;

        Ok(Self { game })
    }

    pub fn register_plugin<P: Plugin>(mut self) -> anyhow::Result<Self> {
        crate::plugin::initialize_plugin::<P>(&mut self.game)?;
        Ok(self)
    }

    pub fn run(self) {
        print_systems(&self.game.system_executor.borrow());
        crate::init::run(self.game);
    }
}

fn print_systems(systems: &SystemExecutor<Game>) {
    let systems: Vec<&str> = systems.system_names().collect();
    log::debug!("---SYSTEMS---\n{:#?}\n", systems);
}

fn build_tokio_runtime() -> Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to create Tokio runtime")
}
