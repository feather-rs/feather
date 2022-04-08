use std::{
    fs::{self},
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context;
use common::Game;
use indexmap::IndexMap;
use quill::{Plugin, PluginInfo, SysResult};
use serde::{Deserialize, Serialize};

/// Manages the loading of plugins at startup time.
pub struct PluginLoader {
    plugins: Vec<RegisteredPlugin>,
    file: PluginsFile,
    file_path: PathBuf,
}

impl PluginLoader {
    pub fn new(plugins_file_path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let plugins_file_path = plugins_file_path.as_ref();
        let file = if plugins_file_path.exists() {
            toml::from_slice(&fs::read(plugins_file_path)?).context("malformed `plugins.toml`")?
        } else {
            PluginsFile::default()
        };
        Ok(Self {
            plugins: Vec::new(),
            file,
            file_path: plugins_file_path.to_path_buf(),
        })
    }

    pub fn register_plugin<P: Plugin>(&mut self, plugin: P) {
        self.plugins.push(RegisteredPlugin::new(plugin));
    }

    pub fn initialize(mut self, game: &mut Game) -> anyhow::Result<()> {
        self.initialize_plugins(game)?;
        self.warn_for_unknown_plugins();

        // Rewrite the plugins.toml file, since its contents may have been updated
        let new_plugins_file = toml::to_string_pretty(&self.file)?;
        fs::write(&self.file_path, new_plugins_file.as_bytes())?;
        Ok(())
    }

    fn initialize_plugins(&mut self, game: &mut Game) -> anyhow::Result<()> {
        for plugin in &mut self.plugins {
            let id = &plugin.info.id;
            let enabled = *self.file.plugins.entry(id.to_string()).or_insert(true);
            if enabled {
                log::info!("Loading plugin {}", plugin.info.name);
                plugin
                    .instance
                    .initialize(game)
                    .with_context(|| format!("failed to initialize plugin {}", plugin.info.name))?;
            } else {
                log::info!(
                    "Plugin {} is disabled and will not be loaded",
                    plugin.info.name
                );
            }
        }
        Ok(())
    }

    fn warn_for_unknown_plugins(&self) {
        for plugin_id in self.file.plugins.keys() {
            let registered = self.plugins.iter().any(|p| p.info.id == plugin_id);
            if !registered {
                log::warn!("plugins.toml refers to a plugin `{}`, but the plugin is not registered in this build of Feather. It will not be loaded.", plugin_id);
            }
        }
    }
}

/// The `plugins.toml` file that defines which registered plugins are enabled/disabled.
#[derive(Debug, Serialize, Deserialize, Default)]
struct PluginsFile {
    plugins: IndexMap<String, bool>,
}

struct RegisteredPlugin {
    /// The `Plugin` instance associated with the plugin.
    instance: Box<dyn ErasedPlugin>,
    /// The plugin metadata.
    info: PluginInfo,
}

impl RegisteredPlugin {
    pub fn new<P: Plugin>(instance: P) -> Self {
        Self {
            info: instance.info(),
            instance: Box::new(instance),
        }
    }
}

/// A type-erased plugin.
trait ErasedPlugin {
    fn initialize(&mut self, game: &mut Game) -> SysResult;
}

impl<P> ErasedPlugin for P
where
    P: Plugin,
{
    fn initialize(&mut self, game: &mut Game) -> SysResult {
        let mut setup = Setup { game };
        let plugin_state = <P as Plugin>::initialize(self, &mut setup)?;
        game.insert_resource(plugin_state);
        Ok(())
    }
}

/// State passed into Plugin::initialize.
struct Setup<'a> {
    game: &'a mut Game,
}

impl<'a, P> quill::Setup<P> for Setup<'a>
where
    P: Plugin,
{
    fn register_system(
        &mut self,
        system: fn(&mut dyn quill::Game, &mut P::State) -> quill::SysResult,
        name: &str,
    ) {
        self.game.system_executor.borrow_mut().add_system_with_name(
            move |game| {
                let resources = Arc::clone(&game.resources);
                let mut state = resources.get_mut::<P::State>()?;
                system(game, &mut state)
            },
            name,
        );
    }

    fn game(&self) -> &dyn quill::Game {
        self.game
    }

    fn game_mut(&mut self) -> &mut dyn quill::Game {
        self.game
    }
}
