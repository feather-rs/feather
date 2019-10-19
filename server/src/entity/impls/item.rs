//! Logic for working with item entities.
use crate::entity::metadata::{self, Metadata};
use crate::entity::{
    ChunkEntities, EntityDestroyEvent, PlayerComponent, PositionComponent, VelocityComponent,
};
use crate::physics::{nearby_entities, PhysicsBuilder};
use crate::player::{
    InventoryComponent, InventoryUpdateEvent, PlayerItemDropEvent, PLAYER_EYE_HEIGHT,
};
use crate::util::{protocol_velocity, Util};
use crate::{TickCount, TPS};
use feather_core::network::packet::implementation::CollectItem;
use feather_core::{Item, ItemStack, Packet};
use rand::Rng;
use shrev::EventChannel;
use smallvec::SmallVec;
use specs::storage::ComponentEvent;
use specs::{
    BitSet, Builder, Component, DenseVecStorage, Entities, Entity, Join, LazyUpdate, Read,
    ReadStorage, ReaderId, System, SystemData, World, WorldExt, Write, WriteStorage,
};

use crate::entity::component::{PacketCreatorComponent, SerializerComponent};
use crate::entity::movement::degrees_to_stops;
use crate::lazy::LazyUpdateExt;
use feather_core::entity::{BaseEntityData, EntityData, ItemData, ItemEntityData};
use feather_core::packet::SpawnObject;
use specs::world::{EntitiesRes, LazyBuilder};
use uuid::Uuid;

/// Component for item entities.
pub struct ItemComponent {
    /// The tick at which this item is collectable
    /// by a player.
    pub collectable_at: u64,
    /// This item's stack.
    pub stack: ItemStack,
}

impl Component for ItemComponent {
    type Storage = DenseVecStorage<Self>;
}

/// System for spawning an item entity when
/// an item is dropped.
///
/// This system listens to `PlayerItemDropEvent`s.
#[derive(Default)]
pub struct ItemSpawnSystem {
    reader: Option<ReaderId<PlayerItemDropEvent>>,
}

impl<'a> System<'a> for ItemSpawnSystem {
    type SystemData = (
        ReadStorage<'a, PositionComponent>,
        Read<'a, LazyUpdate>,
        Entities<'a>,
        Read<'a, EventChannel<PlayerItemDropEvent>>,
        Read<'a, TickCount>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, lazy, entities, item_drop_events, tick) = data;

        let mut rng = rand::thread_rng();

        for event in item_drop_events.read(self.reader.as_mut().unwrap()) {
            // Spawn item entity.

            // Position is player's eye height minus 0.3
            let mut pos = {
                let player_pos = positions.get(event.player).unwrap().current
                    + glm::vec3(0.0, PLAYER_EYE_HEIGHT, 0.0);
                player_pos - glm::vec3(0.0f64, 0.3, 0.0)
            };

            pos.on_ground = false;

            // This velocity calculation was sourced from Glowstone's
            // work. See https://github.com/GlowstoneMC/Glowstone/blob/dev/src/main/java/net/glowstone/entity/GlowHumanEntity.java
            // (method drop(ItemStack stack)) for their code.
            let velocity = {
                let mut vel = pos.direction() * 0.3;
                let rand_offset = 0.02;

                let x = rng.gen_range(0.0, rand_offset) - rand_offset / 2.0;
                let y = rng.gen_range(0.0, 0.12);
                let z = rng.gen_range(0.0, rand_offset) - rand_offset / 2.0;

                vel += glm::vec3(x, y, z);

                vel
            };

            create(&lazy, &entities, event.stack.clone(), tick.0 + TPS)
                .with(PositionComponent {
                    current: pos,
                    previous: pos,
                })
                .with(VelocityComponent(velocity))
                .build();
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.reader = Some(world.fetch_mut::<EventChannel<_>>().register_reader());
    }
}

/// System for merging item entities of the same
/// type.
#[derive(Default)]
pub struct ItemMergeSystem {
    dirty: BitSet,
    reader: Option<ReaderId<ComponentEvent>>,
}

impl<'a> System<'a> for ItemMergeSystem {
    type SystemData = (
        ReadStorage<'a, PositionComponent>,
        ReadStorage<'a, ItemComponent>,
        WriteStorage<'a, Metadata>,
        Write<'a, EventChannel<EntityDestroyEvent>>,
        Read<'a, ChunkEntities>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, item_markers, mut metadatas, mut destroy_events, chunk_entities, entities) =
            data;

        self.dirty.clear();

        for event in positions.channel().read(self.reader.as_mut().unwrap()) {
            match event {
                ComponentEvent::Modified(id) | ComponentEvent::Inserted(id) => {
                    self.dirty.add(*id);
                }
                _ => (),
            }
        }

        let mut metadatas_to_update: SmallVec<[(Entity, Metadata); 2]> = smallvec![];
        // Used to not destroy both entities
        let mut destroyed: SmallVec<[Entity; 2]> = smallvec![];

        for (position, entity, _, _) in (&positions, &entities, &item_markers, &self.dirty).join() {
            if !entities.is_alive(entity) {
                continue;
            }

            if destroyed.iter().any(|x| *x == entity) {
                continue;
            }

            let mut stack = item_stack_from_meta(metadatas.get(entity).unwrap());

            // Find nearby entities and check if they are of the same item
            // type. If so, merge the two item stacks.
            let nearby = nearby_entities(
                &chunk_entities,
                &positions,
                position.current,
                glm::vec3(1.0, 0.5, 1.0),
            );

            for other in nearby {
                // Skip entity if it's dead.
                if !entities.is_alive(other) {
                    continue;
                }

                if other == entity {
                    continue;
                }

                // Skip if it's not an item.
                if item_markers.get(other).is_none() {
                    continue;
                }

                let other_stack = item_stack_from_meta(metadatas.get(other).unwrap());

                if other_stack.ty != stack.ty {
                    continue;
                }

                // Merge two stacks.
                // This works by deleting `other` and adding
                // together the amounts of the two item stacks.
                entities.delete(other).unwrap();

                let event = EntityDestroyEvent { entity: other };
                destroy_events.single_write(event);

                // TODO this could overflow...
                stack.amount += other_stack.amount;

                metadatas_to_update.push((entity, item_meta(stack.clone())));
                destroyed.push(other);
            }
        }

        metadatas_to_update.into_iter().for_each(|(entity, meta)| {
            metadatas.insert(entity, meta).unwrap();
        });
    }

    flagged_setup_impl!(PositionComponent, reader);
}

/// System for collecting items when a player comes
/// near them.
#[derive(Default)]
pub struct ItemCollectSystem {
    dirty: BitSet,
    reader: Option<ReaderId<ComponentEvent>>,
}

impl<'a> System<'a> for ItemCollectSystem {
    type SystemData = (
        WriteStorage<'a, InventoryComponent>,
        ReadStorage<'a, PositionComponent>,
        ReadStorage<'a, PlayerComponent>,
        ReadStorage<'a, ItemComponent>,
        WriteStorage<'a, Metadata>,
        Write<'a, EventChannel<InventoryUpdateEvent>>,
        Write<'a, EventChannel<EntityDestroyEvent>>,
        Read<'a, ChunkEntities>,
        Read<'a, Util>,
        Read<'a, TickCount>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut inventories,
            positions,
            players,
            items,
            mut metadatas,
            mut inventory_events,
            mut destroy_events,
            chunk_entities,
            util,
            tick,
            entities,
        ) = data;

        self.dirty.clear();

        read_flagged_events!(positions, self.reader, self.dirty);

        // For each player who has moved this tick,
        // look for nearby items.
        // We need to keep track of which items
        // have already been collected to avoid
        // having the same item being collected
        // by two players at once; this would
        // cause dupe exploits.
        let mut collected_items: SmallVec<[Entity; 4]> = smallvec![];

        for (position, inventory, player, _, _) in (
            &positions,
            &mut inventories,
            &entities,
            &players,
            &self.dirty,
        )
            .join()
        {
            let nearby = nearby_entities(
                &chunk_entities,
                &positions,
                position.current,
                glm::vec3(1.0, 0.5, 1.0),
            );

            for other in nearby {
                // If it's not an item, skip.
                let item = continue_if_none!(items.get(other));

                // Check if the item can be picked up yet.
                if item.collectable_at > tick.0 {
                    continue;
                }

                // If the item has already been collected, don't try it.
                if collected_items.iter().any(|x| *x == other) {
                    continue;
                }

                // Attempt to collect the item.
                let mut stack = item_stack_from_meta(metadatas.get(other).unwrap());
                let (affected_slots, amount_left) = inventory.collect_item(stack.clone());

                // Broadcast Collect Item packet, which gives an animation.
                let packet = CollectItem {
                    collected: other.id() as i32,
                    collector: player.id() as i32,
                    count: i32::from(stack.amount - amount_left),
                };
                util.broadcast_entity_update(player, packet, None);

                if amount_left == 0 {
                    entities.delete(other).unwrap();
                    collected_items.push(other);

                    let event = EntityDestroyEvent { entity: other };
                    destroy_events.single_write(event);
                } else {
                    stack.amount = amount_left;
                    let meta = item_meta(stack);
                    metadatas.insert(other, meta).unwrap();
                }

                // Trigger inventory update event.
                let event = InventoryUpdateEvent {
                    slots: affected_slots,
                    player,
                };
                inventory_events.single_write(event);
            }
        }
    }

    flagged_setup_impl!(PositionComponent, reader);
}

pub fn create<'a>(
    lazy: &'a LazyUpdate,
    entities: &EntitiesRes,
    stack: ItemStack,
    collectable_at: u64,
) -> LazyBuilder<'a> {
    let meta = {
        let mut meta_item = crate::entity::metadata::Item::default();
        meta_item.set_item(Some(stack.clone()));
        Metadata::Item(meta_item)
    };

    lazy.spawn_entity(entities)
        .with(ItemComponent {
            stack,
            collectable_at,
        })
        .with(
            PhysicsBuilder::new()
                .bbox(0.25, 0.25, 0.25)
                .gravity(-0.04)
                .drag(0.98)
                .build(),
        )
        .with(VelocityComponent::default())
        .with(meta)
        .with(PacketCreatorComponent(&create_packet))
        .with(SerializerComponent(&serialize))
}

pub fn create_from_data(
    lazy: &LazyUpdate,
    entities: &EntitiesRes,
    data: &ItemEntityData,
    tick: &TickCount,
) -> Option<Entity> {
    let pos = data.entity.read_position()?;
    let vel = data.entity.read_velocity()?;

    let stack = ItemStack::new(Item::from_identifier(&data.item.item)?, data.item.count);

    let collectable_at = data.pickup_delay as u64 + tick.0;

    Some(
        create(lazy, entities, stack, collectable_at)
            .with(PositionComponent {
                current: pos,
                previous: pos,
            })
            .with(VelocityComponent(vel))
            .build(),
    )
}

fn create_packet(world: &World, entity: Entity) -> Box<dyn Packet> {
    let positions = world.read_component::<PositionComponent>();
    let velocities = world.read_component::<VelocityComponent>();

    let position = positions.get(entity).unwrap().current;
    let (velocity_x, velocity_y, velocity_z) = protocol_velocity(velocities.get(entity).unwrap().0);

    let packet = SpawnObject {
        entity_id: entity.id() as i32,
        object_uuid: Uuid::new_v4(),
        ty: 2, // Type 2 for item stack
        x: position.x,
        y: position.y,
        z: position.z,
        pitch: degrees_to_stops(position.pitch),
        yaw: degrees_to_stops(position.yaw),
        data: 1, // Has velocity
        velocity_x,
        velocity_y,
        velocity_z,
    };

    Box::new(packet)
}

fn serialize(world: &World, entity: Entity) -> EntityData {
    let positions = world.read_component::<PositionComponent>();
    let velocities = world.read_component::<VelocityComponent>();
    let items = world.read_component::<ItemComponent>();

    let item = items.get(entity).unwrap();
    let position = positions.get(entity).unwrap();
    let velocity = velocities.get(entity).unwrap();

    EntityData::Item(ItemEntityData {
        entity: BaseEntityData::new(position.current, velocity.0),
        age: 0,          // TODO
        pickup_delay: 0, // TODO
        item: ItemData {
            item: item.stack.ty.identifier().to_string(),
            count: item.stack.amount,
        },
    })
}

pub fn item_stack_from_meta(meta: &Metadata) -> ItemStack {
    match meta {
        Metadata::Item(item) => item.item().unwrap().clone(),
        _ => panic!(),
    }
}

pub fn item_meta(stack: ItemStack) -> Metadata {
    let mut item = metadata::Item::default();
    item.set_item(Some(stack));
    Metadata::Item(item)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::{ChunkEntityUpdateSystem, EntitySpawnEvent};
    use crate::testframework as t;
    use feather_core::inventory::SLOT_HOTBAR_OFFSET;
    use feather_core::network::cast_packet;
    use feather_core::world::Position;
    use feather_core::{Item, ItemStack, PacketType};
    use specs::WorldExt;

    #[test]
    fn test_item_spawn_system() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);
        t::set_entity_pos(&w, player.entity, position!(0.0, 1.0, 0.0));

        let stack = ItemStack::new(Item::AcaciaBoat, 4);

        let mut entity_spawn_reader = t::reader(&w);

        let event = PlayerItemDropEvent {
            slot: None,
            stack,
            player: player.entity,
        };
        t::trigger_event(&w, event);

        d.dispatch(&w);
        w.maintain();

        // Confirm event was triggered
        let events = t::triggered_events::<EntitySpawnEvent>(&w, &mut entity_spawn_reader);
        assert_eq!(events.len(), 1);
        let first = events.first().unwrap();
        let entity = first.entity;

        // Check position
        let pos = t::entity_pos(&w, entity);
        assert_float_eq!(pos.x, 0.0);
        assert_float_eq!(pos.z, 0.0);

        // Confirm that velocity was created
        let _vel = t::entity_vel(&w, entity).unwrap();
    }

    #[test]
    fn test_item_merge_system() {
        let (mut w, mut d) = t::builder()
            .with_dep(ItemMergeSystem::default(), "item_merge", &[])
            .build();

        let item1 = create(
            &w.fetch(),
            &w.fetch(),
            ItemStack::new(Item::EnderPearl, 4),
            0,
        )
        .with(PositionComponent::default())
        .build();
        let item2 = create(
            &w.fetch(),
            &w.fetch(),
            ItemStack::new(Item::EnderPearl, 7),
            0,
        )
        .with(PositionComponent::default())
        .build();

        let mut updater = ChunkEntityUpdateSystem::default();
        updater.setup(&mut w);

        w.maintain();

        // Update chunk entities so `nearby_entities` works
        specs::RunNow::run_now(&mut updater, &w);

        d.dispatch(&w);
        w.maintain();

        assert!(!w.is_alive(item2));
        assert!(w.is_alive(item1));

        let metadatas = w.read_component::<Metadata>();
        let metadata = metadatas.get(item1).unwrap();

        let stack = item_stack_from_meta(&metadata);
        assert_eq!(stack.ty, Item::EnderPearl);
        assert_eq!(stack.amount, 11);
    }

    #[test]
    fn test_item_collect_system() {
        let (mut w, mut d) = t::builder()
            .with_dep(ItemCollectSystem::default(), "", &[])
            .build();

        let player = t::add_player(&mut w);
        let stack = ItemStack::new(Item::String, 4);
        let item = create(&w.fetch(), &w.fetch(), stack.clone(), 0)
            .with(PositionComponent::default())
            .build();

        let mut destroy_reader = t::reader(&w);

        // Allow item to be collected
        w.fetch_mut::<TickCount>().0 = 0;

        let mut updater = ChunkEntityUpdateSystem::default();
        updater.setup(&mut w);

        w.maintain();

        // Update chunk entities so `nearby_entities` works

        specs::RunNow::run_now(&mut updater, &w);

        d.dispatch(&w);
        w.maintain();

        let destroy_events = t::triggered_events::<EntityDestroyEvent>(&w, &mut destroy_reader);
        let first = destroy_events.first().unwrap();
        assert_eq!(first.entity, item);

        assert!(!w.is_alive(item));

        let inventories = w.read_component::<InventoryComponent>();
        let inventory = inventories.get(player.entity).unwrap();

        assert_eq!(inventory.item_at(SLOT_HOTBAR_OFFSET), Some(&stack));

        let packet = t::assert_packet_received(&player, PacketType::CollectItem);
        let packet = cast_packet::<CollectItem>(&*packet);

        assert_eq!(packet.collector, player.entity.id() as i32);
        assert_eq!(packet.collected, item.id() as i32);
        assert_eq!(packet.count, 4);
    }
}
