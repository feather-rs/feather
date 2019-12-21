use crossbeam::queue::SegQueue;
use legion::entity::Entity;
use legion::storage::{Component, Tag};
use legion::world::World;
use smallvec::SmallVec;
use tonks::Scheduler;

pub trait LazyFnWithScheduler: FnOnce(&mut World, &mut Scheduler) + Send {}
impl<F> LazyFnWithScheduler for F where F: FnOnce(&mut World, &mut Scheduler) + Send {}

pub trait LazyFn: FnOnce(&mut World) + Send {}
impl<F> LazyFn for F where F: FnOnce(&mut World) + Send {}

pub trait LazyEntityFn: FnOnce(&mut World, &mut Scheduler, Entity) + Send {}
impl<F> LazyEntityFn for F where F: FnOnce(&mut World, &mut Scheduler, Entity) + Send {}

/// Resource which allows lazy creation of entities
/// or execution of functions with world access.
#[derive(Default, Resource)]
pub struct Lazy {
    /// Internal queue of actions to perform.
    queue: SegQueue<Action>,
}

impl Lazy {
    /// Lazily executes a closure with world access.
    pub fn exec(&self, f: impl FnOnce(&mut World) + Send + 'static) {
        self.exec_with_scheduler(move |world, _| f(world));
    }

    /// Lazily executes a closure with world and scheduler (resource)
    /// access.
    pub fn exec_with_scheduler(&self, f: impl FnOnce(&mut World, &mut Scheduler) + Send + 'static) {
        self.queue.push(Action::Exec(Box::new(f)));
    }

    /// Creates an `EntityBuilder` which can be used to lazily
    /// create an entity.
    pub fn create_entity(&self) -> EntityBuilder {
        EntityBuilder {
            lazy: self,
            fns: smallvec![],
        }
    }

    /// Performs all queued actions.
    pub fn flush(&self, world: &mut World, scheduler: &mut Scheduler) {
        while let Ok(action) = self.queue.pop() {
            match action {
                Action::Exec(f) => f(world, scheduler),
            }
        }
    }
}

/// An action which the lazy updater may perform.
enum Action {
    Exec(Box<dyn LazyFnWithScheduler>),
}

/// Builder for lazily creating entities.
pub struct EntityBuilder<'a> {
    lazy: &'a Lazy,
    fns: SmallVec<[Box<dyn LazyEntityFn>; 8]>,
}

impl<'a> EntityBuilder<'a> {
    pub fn with_component<C: Component>(mut self, component: C) -> Self {
        self.fns.push(Box::new(
            move |world: &mut World, _: &mut Scheduler, entity: Entity| {
                world.add_component(entity, component);
            },
        ));
        self
    }

    pub fn with_tag<T: Tag>(mut self, tag: T) -> Self {
        self.fns.push(Box::new(
            move |world: &mut World, _: &mut Scheduler, entity: Entity| {
                world.add_tag(entity, tag);
            },
        ));
        self
    }

    /// Executes a function with the entity after it is created.
    pub fn with_exec(
        mut self,
        f: impl FnOnce(&mut World, &mut Scheduler, Entity) + Send + 'static,
    ) -> Self {
        self.fns.push(Box::new(f));
        self
    }

    pub fn build(self) {
        let fns = self.fns;
        self.lazy.exec_with_scheduler(move |world, scheduler| {
            let entity = world.insert((), [()].iter().copied())[0];

            for f in fns {
                f(world, scheduler, entity);
            }
        })
    }
}
