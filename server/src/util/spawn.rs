use crate::entity::ItemComponent;
use crate::entity::Metadata;
use crate::entity::{EntitySpawnEvent, EntityType, PositionComponent, VelocityComponent};
use crate::util::Util;
use crate::{TickCount, TPS};
use crossbeam::queue::SegQueue;
use feather_core::{ItemStack, Position};
use glm::Vec3;
use shrev::EventChannel;
use specs::{Entities, Read, System, Write, WriteStorage};

/// This type implements a convenient
/// way to spawn entities without having to
/// add a ton of system dependencies.
///
/// It works by queueing mob spawn requests
/// in an internal vector and lazily
/// creating the entities during the
/// handling phase of the dispatcher.
///
/// # Note
/// This resource is used as a subset
/// of the `Util` struct. Never use the `Spawner`
/// directly.
#[derive(Default, Debug)]
pub struct Spawner {
    /// The internal queue of spawn requests.
    queue: SegQueue<SpawnRequest>,
}

impl Spawner {
    /// Queues an item entity to be spawned.
    pub fn spawn_item(&self, position: Position, velocity: Vec3, item: ItemStack) {
        let meta = {
            let mut meta_item = crate::entity::metadata::Item::default();
            meta_item.set_item(Some(item.clone()));
            Metadata::Item(meta_item)
        };
        let request = SpawnRequest {
            ty: EntityType::Item,
            position,
            velocity,
            meta,

            extra: Extra::Item(item),
        };

        self.queue.push(request);
    }
}

#[derive(Debug, Clone)]
struct SpawnRequest {
    ty: EntityType,
    position: Position,
    velocity: Vec3,
    meta: Metadata,

    extra: Extra,
}

#[derive(Debug, Clone)]
enum Extra {
    Item(ItemStack),
}

/// System for spawning queued requests in the `Spawner`.
pub struct SpawnerSystem;

impl<'a> System<'a> for SpawnerSystem {
    type SystemData = (
        Read<'a, Util>,
        WriteStorage<'a, PositionComponent>,
        WriteStorage<'a, VelocityComponent>,
        WriteStorage<'a, Metadata>,
        WriteStorage<'a, EntityType>,
        WriteStorage<'a, ItemComponent>,
        Write<'a, EventChannel<EntitySpawnEvent>>,
        Read<'a, TickCount>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            util,
            mut positions,
            mut velocities,
            mut metadatas,
            mut types,
            mut item_markers,
            mut spawn_events,
            tick,
            entities,
        ) = data;

        // Handle spawn requests
        while let Ok(request) = util.spawner.queue.pop() {
            let entity = entities.create();

            positions
                .insert(
                    entity,
                    PositionComponent {
                        current: request.position,
                        previous: request.position,
                    },
                )
                .unwrap();
            velocities
                .insert(entity, VelocityComponent(request.velocity))
                .unwrap();
            metadatas.insert(entity, request.meta).unwrap();
            types.insert(entity, request.ty).unwrap();

            match request.ty {
                EntityType::Item => {
                    item_markers
                        .insert(
                            entity,
                            ItemComponent {
                                collectable_at: tick.0 + TPS,
                            },
                        )
                        .unwrap();
                }
                _ => unimplemented!(),
            }

            // Trigger event
            let event = EntitySpawnEvent {
                entity,
                ty: request.ty,
            };
            spawn_events.single_write(event);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::EntitySpawnEvent;
    use crate::testframework as t;
    use feather_core::Item;

    #[test]
    fn test_spawn_item() {
        let spawner = Spawner::default();

        let position = position!(0.0, 10.0, 1.04);
        let velocity = glm::vec3(104.0, 4.0, 10.0);
        let item = ItemStack::new(Item::EnderPearl, 4);

        spawner.spawn_item(position, velocity, item);

        let request = spawner.queue.pop().unwrap();
        assert_eq!(request.ty, EntityType::Item);
        assert_eq!(request.position, position);
        assert_eq!(request.velocity, velocity);
    }

    #[test]
    fn test_spawner_system() {
        let (w, mut d) = t::builder().with(SpawnerSystem, "").build();

        let position = position!(0.0, 10.0, 1.04);
        let velocity = glm::vec3(104.0, 4.0, 10.0);
        let item = ItemStack::new(Item::EnderPearl, 4);

        let mut reader = t::reader(&w);

        {
            let util = w.fetch::<Util>();
            util.spawn_item(position, velocity, item);
        }

        d.dispatch(&w);

        let events = t::triggered_events::<EntitySpawnEvent>(&w, &mut reader);
        assert_eq!(events.len(), 1);

        let first = events.first().unwrap();
        assert_eq!(first.ty, EntityType::Item);
    }
}
