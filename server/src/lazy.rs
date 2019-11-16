use crossbeam::queue::SegQueue;
use legion::entity::Entity;
use legion::storage::{Component, Tag};
use legion::world::World;
use smallvec::SmallVec;
use tonks::Scheduler;

/// Resource which allows lazy creation of entities
/// or execution of functions with world access.
pub struct Lazy {
    /// Internal queue of actions to perform.
    queue: SegQueue<Action>,
}

impl Lazy {
    /// Lazily executes a closure with world access.
    pub fn exec(&self, f: impl FnOnce(&mut World)) {
        self.exec_with_scheduler(move |world, _| f(world));
    }

    /// Lazily executes a closure with world and scheduler (resource)
    /// access.
    pub fn exec_with_scheduler(&self, f: impl FnOnce(&mut World, &mut Scheduler)) {
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
    Exec(Box<dyn FnOnce(&mut World, &mut Scheduler)>),
}

/// Builder for lazily creating entities.
pub struct EntityBuilder<'a> {
    lazy: &'a Lazy,
    fns: SmallVec<[Box<dyn FnOnce(&mut World, Entity)>; 8]>,
}

impl<'a> EntityBuilder<'a> {
    pub fn with_component<C: Component>(mut self, component: C) -> Self {
        self.fns.push(move |world, entity| {
            world.add_component(entity, component);
        });
        self
    }

    pub fn with_tag<T: Tag>(mut self, tag: T) -> Self {
        self.fns.push(move |world, entity| {
            world.add_tag(entity, tag);
        });
        self
    }

    pub fn build(self) {
        self.lazy.exec(move |world| {
            let entity = world.insert((), [()].iter().copied())[0];

            for f in self.fns {
                f(world, entity);
            }
        })
    }
}
