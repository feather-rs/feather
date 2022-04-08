use vane::{Resources, SysResult};

use crate::Game;

/// Context passed to `Plugin::initialize`.
pub trait Setup<P: Plugin> {
    /// Registers a system.
    fn register_system(
        &mut self,
        system: fn(&mut dyn Game, &mut P::State) -> SysResult,
        name: &str,
    );

    /// Gets the `Game`.
    fn game(&self) -> &dyn Game;

    /// Mutably gets the `Game`.
    fn game_mut(&mut self) -> &mut dyn Game;

    /// Gets the `Resources`.
    fn resources(&self) -> &Resources {
        self.game().resources()
    }

    /// Mutably gets the `Resources`.
    fn resources_mut(&mut self) -> &mut Resources {
        self.game_mut().resources_mut()
    }
}

/// Represents a plugin loaded at startup.
///
/// Every plugin should have a unit struct implementing this trait.
pub trait Plugin: 'static {
    /// A plugin state passed as the second parameter to all plugin systems.
    type State: 'static;

    /// Gets the plugin's info.
    fn info(&self) -> PluginInfo;

    /// Called at plugin load time.
    ///
    /// You should register systems and insert resources in this method.
    fn initialize(&mut self, setup: &mut dyn Setup<Self>) -> SysResult<Self::State>;
}

/// Metadata associated with a plugin.
#[derive(Debug)]
pub struct PluginInfo {
    /// Display name of the plugin in PascalCase.
    pub name: &'static str,
    /// ID of the plugin in snake_case.
    pub id: &'static str,
}
