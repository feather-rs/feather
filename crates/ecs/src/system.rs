//! System execution, using a simple "systems as functions" model.

use std::{any::type_name, marker::PhantomData, sync::Arc};

use resources::Resources;

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
/// That said, these errors should never happen in production.
pub type SysResult<T = ()> = anyhow::Result<T>;

type SystemFn<Input> = Box<dyn FnMut(&mut Input) -> SysResult>;

struct System<Input> {
    function: SystemFn<Input>,
    name: String,
}

impl<Input> System<Input> {
    fn from_fn<F: FnMut(&mut Input) -> SysResult + 'static>(f: F) -> Self {
        Self {
            function: Box::new(f),
            name: type_name::<F>().to_owned(),
        }
    }
}

/// A type containing a `Resources`.
pub trait HasResources {
    fn resources(&self) -> Arc<Resources>;
}

/// An executor for systems.
///
/// This executor contains a sequence of systems, each
/// of which is simply a function taking an `&mut Input`.
///
/// Systems may belong to _groups_, where each system
/// gets an additional parameter representing the group state.
/// For example, the `Server` group has state contained in the `Server`
/// struct, so all its systems get `Server` as an extra parameter.
///
/// Systems run sequentially in the order they are added to the executor.
pub struct SystemExecutor<Input> {
    systems: Vec<System<Input>>,
}

impl<Input> Default for SystemExecutor<Input> {
    fn default() -> Self {
        Self {
            systems: Vec::new(),
        }
    }
}

impl<Input> SystemExecutor<Input> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a system to the executor.
    pub fn add_system(
        &mut self,
        system: impl FnMut(&mut Input) -> SysResult + 'static,
    ) -> &mut Self {
        let system = System::from_fn(system);
        self.systems.push(system);
        self
    }

    /// Begins a group with the provided group state type.
    ///
    /// The group state must be added to the `resources`.
    pub fn group<State>(&mut self) -> GroupBuilder<Input, State>
    where
        Input: HasResources,
    {
        GroupBuilder {
            systems: self,
            _marker: PhantomData,
        }
    }

    /// Runs all systems in order.
    ///
    /// Errors are logged using the `log` crate.
    pub fn run(&mut self, input: &mut Input) {
        for system in &mut self.systems {
            let result = (system.function)(input);
            if let Err(e) = result {
                log::error!(
                    "System {} returned an error; this is a bug: {:?}",
                    system.name,
                    e
                );
            }
        }
    }
}

/// Builder for a group. Created with [`SystemExecutor::group`].
pub struct GroupBuilder<'a, Input, State> {
    systems: &'a mut SystemExecutor<Input>,
    _marker: PhantomData<State>,
}

impl<'a, Input, State> GroupBuilder<'a, Input, State>
where
    Input: HasResources + 'static,
    State: Send + Sync + 'static,
{
    /// Adds a system to the group.
    pub fn add_system(
        &mut self,
        system: impl FnMut(&mut Input, &mut State) -> SysResult + 'static,
    ) -> &mut Self {
        let function = Self::make_function(system);
        self.systems.add_system(function);
        self
    }

    fn make_function(
        mut system: impl FnMut(&mut Input, &mut State) -> SysResult + 'static,
    ) -> impl FnMut(&mut Input) -> SysResult + 'static {
        move |input: &mut Input| {
            let resources = input.resources();
            let mut state = resources
                .get_mut::<State>()
                .expect("missing state resource for group");
            system(input, &mut *state)
        }
    }
}
