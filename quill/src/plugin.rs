use vane::{Resources, SysResult};

use crate::Game;

/// Context passed to `Plugin::initialize`.
pub trait Setup {
    /// Registers a system.
    fn register_system(&mut self, system: fn(&mut dyn Game) -> SysResult);

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
/// Every plugin should have a struct implementing this trait.
pub trait Plugin: 'static {
    /// Gets the plugin's name.
    fn name(&self) -> &'static str;

    /// Called at plugin load time.
    ///
    /// You should register systems and insert resources in this method.
    fn initialize(&mut self, setup: &mut dyn Setup) -> SysResult;
}
