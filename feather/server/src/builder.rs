use common::Game;
use feather_world_format_plugin::FeatherWorldFormat;
use quill::Plugin;
use tokio::runtime::Runtime;
use vane::SystemExecutor;

use crate::plugin::PluginLoader;

pub struct ServerBuilder {
    game: Game,
    plugin_loader: PluginLoader,
}

impl ServerBuilder {
    pub fn new() -> anyhow::Result<Self> {
        let runtime = build_tokio_runtime();
        let game = crate::init::create_game(runtime)?;
        let plugin_loader = PluginLoader::new("plugins.toml")?;

        Ok(Self {
            game,
            plugin_loader,
        })
    }

    pub fn register_default_plugins(self) -> Self {
        self.register_plugin(FeatherWorldFormat)
    }

    pub fn register_plugin<P: Plugin>(mut self, plugin: P) -> Self {
        self.plugin_loader.register_plugin(plugin);
        self
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        self.plugin_loader.initialize(&mut self.game)?;
        print_systems(&self.game.system_executor.borrow());
        crate::init::run(self.game)
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
