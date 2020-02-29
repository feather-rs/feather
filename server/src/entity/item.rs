//! Handling of item entities.

use crate::entity::{EntityId, EntityMoveEvent, SpawnPacketCreator, Velocity};
use crate::lazy::EntityBuilder;
use crate::metadata::Metadata;
use crate::p_inventory::{EntityInventory, InventoryUpdateEvent};
use crate::physics::{nearby_entities, PhysicsBuilder};
use crate::player::PLAYER_EYE_HEIGHT;
use crate::state::State;
use crate::util::{degrees_to_stops, protocol_velocity};
use crate::{entity, TickCount, TPS};
use feather_core::inventory::SlotIndex;
use feather_core::network::packet::implementation::SpawnObject;
use feather_core::{ItemStack, Packet, Position};
use legion::entity::Entity;
use legion::query::{Read, Write};
use rand::Rng;
use std::ops::DerefMut;
use std::sync::atomic::{AtomicBool, Ordering};
use tonks::{EntityAccessor, PreparedWorld, Query, Trigger};
use uuid::Uuid;

/// Event triggered when an item is dropped.
///
/// Before this event is triggered, the item
/// is removed from the player's inventory.
#[derive(Debug, Clone)]
pub struct ItemDropEvent {
    /// The slot from which the item was dropped,
    /// if known.
    pub slot: Option<SlotIndex>,
    /// The item stack which was dropped.
    pub stack: ItemStack,
    /// The player who dropped the item.
    pub player: Entity,
}

/// Component storing the tick at which an item becomes collectable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CollectableAt(pub u64);

/// Component storing if an item stack has been collected and queued for removal.
pub struct IsRemoved(AtomicBool);

// Item stack of an item entity is stored in `ItemStack` component

/// System for spawning an item entity when
/// an item is dropped.
#[event_handler]
pub fn item_spawn(
    event: &ItemDropEvent,
    state: &State,
    _query: &mut Query<Read<Position>>,
    world: &mut PreparedWorld,
    tick: &TickCount,
) {
    let mut rng = rand::thread_rng();

    // Spawn item entity.

    // Position is player's eye height minus 0.3
    let mut pos = {
        let player_pos = *world.get_component::<Position>(event.player).unwrap()
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

    create(state, pos, event.stack.clone(), tick.0 + TPS)
        .with_component(Velocity(velocity))
        .build();
}

/// System to add items to entity inventories.
#[event_handler]
pub fn item_collect(
    events: &[EntityMoveEvent],
    state: &State,
    _query: &mut Query<(
        Write<Metadata>,
        Read<IsRemoved>,
        Write<ItemStack>,
        Write<EntityInventory>,
        Read<Position>,
    )>,
    world: &mut PreparedWorld,
    trigger: &mut Trigger<InventoryUpdateEvent>,
) {
    // TODO: switch to par_iter
    events.iter().for_each(|event: &EntityMoveEvent| {
        if world
            .get_component::<EntityInventory>(event.entity)
            .is_none()
        {
            return;
        }

        let pos = *world.get_component::<Position>(event.entity).unwrap();
        // Find nearby items.
        let nearby_entities =
            nearby_entities(&state.chunk_entities, world, pos, glm::vec3(1.0, 0.5, 1.0));

        for other in nearby_entities {
            if let Some(item_stack) = world.get_component::<ItemStack>(other).map(|item| *item) {
                // Ensure that this item hasn't already been collected, to avoid duplication.
                {
                    let is_removed = world.get_component::<IsRemoved>(other).unwrap();
                    if is_removed
                        .0
                        .compare_and_swap(false, true, Ordering::Relaxed)
                    {
                        continue;
                    }
                }

                let (affected_slots, items_left) = {
                    let mut inventory = world
                        .get_component_mut::<EntityInventory>(event.entity)
                        .unwrap();
                    inventory.collect_item(item_stack)
                };

                trigger.trigger(InventoryUpdateEvent {
                    slots: affected_slots,
                    player: event.entity,
                });

                if items_left == 0 {
                    state.delete_entity(other);
                } else {
                    // Update item stack
                    let new_stack = ItemStack::new(item_stack.ty, items_left);
                    *world.get_component_mut::<ItemStack>(other).unwrap() = new_stack;
                    match world
                        .get_component_mut::<Metadata>(other)
                        .unwrap()
                        .deref_mut()
                    {
                        Metadata::Item(ref mut meta_item) => meta_item.set_item(Some(new_stack)),
                        _ => unreachable!(),
                    }

                    world
                        .get_component::<IsRemoved>(other)
                        .unwrap()
                        .0
                        .store(false, Ordering::Relaxed);
                }
            }
        }
    });
}

/// Returns an entity builder to create an item entity
/// with the given stack and collectable tick.
pub fn create(
    state: &State,
    pos: Position,
    stack: ItemStack,
    collectable_at: u64,
) -> EntityBuilder {
    let meta = {
        let mut meta_item = crate::metadata::Item::default();
        meta_item.set_item(Some(stack.clone()));
        Metadata::Item(meta_item)
    };

    entity::base(state, pos)
        .with_component(stack)
        .with_component(CollectableAt(collectable_at))
        .with_component(SpawnPacketCreator(&create_spawn_packet))
        .with_component(meta)
        .with_component(IsRemoved(AtomicBool::new(false)))
        .with_component(
            PhysicsBuilder::new()
                .bbox(0.25, 0.25, 0.25)
                .drag(0.98)
                .gravity(-0.04)
                .build(),
        )
}

fn create_spawn_packet(accessor: &EntityAccessor, world: &PreparedWorld) -> Box<dyn Packet> {
    let position = *accessor.get_component::<Position>(world).unwrap();
    let velocity = *accessor.get_component::<Velocity>(world).unwrap();
    let entity_id = accessor.get_component::<EntityId>(world).unwrap().0;

    let (velocity_x, velocity_y, velocity_z) = protocol_velocity(velocity.0);

    let packet = SpawnObject {
        entity_id,
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
