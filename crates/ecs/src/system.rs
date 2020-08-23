//! System execution, using a simple "systems as functions" model.

use std::collections::BTreeMap;

/// The result type returned by a system function.
///
/// When a system encounters an internal error, it should return
/// an error instead of panicking. The system executor will then
/// log an error message to the console and attempt to gracefully
/// recover.
///
/// Examples of internal errors include:
/// * An entity was missing a component which it was expected to have.
/// (For example, all entities have a `Position` component; if an entity
/// is missing it, then that is valid grounds for a system to return an error.)
/// * IO errors
///
/// That said, errors should never happen in production.
pub type SysResult<T = ()> = anyhow::Result<T>;

/// A stage in the system executor.
///
/// Each system belongs to a stage. Each tick,
/// the systems in each stage are executor in the
/// order of the stages.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stage {
    /// Called before main gameplay logic runs.
    Pre,
    /// Should handle all gameplay logic.
    Tick,
    /// Should be used to handle events.
    HandleEvents,
    /// Should be used for packet broadcasting on the
    /// server side and packet sending on the client.
    SendPackets,
    /// Should be used to clean up / reset resources
    /// at the end of the tick.
    CleanUp,
}

/// The type of a system function.
pub type SystemFn<Input> = fn(&mut Input) -> SysResult;

/// An executor for systems.
///
/// This executor stores an array of systems registered
/// for each stage. When `tick()` is called, all systems are invoked
/// sequentially.
pub struct SystemExecutor<Input> {
    stages: BTreeMap<Stage, Vec<SystemFn<Input>>>,
}

impl<Input> Default for SystemExecutor<Input> {
    fn default() -> Self {
        Self {
            stages: BTreeMap::new(),
        }
    }
}

impl<Input> SystemExecutor<Input> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a system to the executor.
    pub fn add_system(&mut self, stage: Stage, system: SystemFn<Input>) -> &mut Self {
        self.stages.entry(stage).or_default().push(system);
        self
    }

    /// Invokes all systems in order.
    pub fn tick(&self, input: &mut Input) {
        for (&stage, systems) in &self.stages {
            for (index, system) in systems.iter().copied().enumerate() {
                if let Err(e) = system(input) {
                    // TODO: figure out a way to get the name of the system function.
                    log::error!(
                        "System #{} of stage {:?} failed to execute: {:?}\nThis is a bug.",
                        index,
                        stage,
                        e
                    );
                }
            }
        }
    }
}
