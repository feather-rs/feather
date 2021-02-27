//! Handling of item entities.

use feather_core::anvil::entity::{
    BaseEntityData, EntityData, EntityDataKind, ItemData, ItemEntityData, ItemNbt,
};
use feather_core::entitymeta::{EntityMetadata, META_INDEX_ITEM_SLOT};
use feather_core::inventory::Inventory;
use feather_core::items::{Item, ItemStack};
use feather_core::network::packets::SpawnObject;
use feather_core::network::Packet;
use feather_core::util::{Position, Vec3d};
use feather_server_types::{
    ComponentSerializer, Dead, EntityLoaderRegistration, EntitySpawnEvent, Game,
    InventoryUpdateEvent, ItemCollectEvent, ItemDropEvent, NetworkId, PhysicsBuilder, Player,
    SpawnPacketCreator, Uuid, Velocity, PLAYER_EYE_HEIGHT, TPS,
};
use feather_server_util::{degrees_to_stops, nearby_entities, protocol_velocity};
use fecs::{component, EntityBuilder, EntityRef, IntoQuery, Read, World, Write};
use parking_lot::Mutex;
use rand::Rng;
use std::sync::atomic::{AtomicBool, Ordering};

/// Component which stores the world time at which an item
/// will be collectable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CollectableAt(u64);

/// Component used to store whether an item has been collected/
/// removed on a given tick. Used by `item_collect` and `item_merge`
/// systems.
#[derive(Debug)]
struct IsRemoved(AtomicBool);

inventory::submit! {
    EntityLoaderRegistration::new(EntityDataKind::Item, &load)
}

/// Handler for spawning an item entity when
/// an item is dropped.
#[fecs::event_handler]
pub fn on_item_drop_spawn_item_entity(event: &ItemDropEvent, game: &mut Game, world: &mut World) {
    // Spawn item entity.

    // Position is player's eye height minus 0.3
    let mut pos = {
        let player_pos =
            *world.get::<Position>(event.player) + glm::vec3(0.0, PLAYER_EYE_HEIGHT, 0.0);
        player_pos - glm::vec3(0.0f64, 0.3, 0.0)
    };

    pos.on_ground = false;

    let mut rng = game.rng();

    // This velocity calculation was sourced from Glowstone's
    // work. See https://github.com/GlowstoneMC/Glowstone/blob/dev/src/main/java/net/glowstone/entity/GlowHumanEntity.java
    // (method drop(ItemStack stack)) for their code.
    let velocity = {
        let mut vel = glm::DVec3::from_column_slice(&(pos.direction() * 0.3).into_array());
        let rand_offset = 0.02;

        let x = rng.gen_range(0.0, rand_offset) - rand_offset / 2.0;
        let y = rng.gen_range(0.0, 0.12);
        let z = rng.gen_range(0.0, rand_offset) - rand_offset / 2.0;

        vel += glm::vec3(x, y, z);

        vel
    };

    drop(rng);

    let entity = create(event.stack, game.tick_count + TPS)
        .with(pos)
        .with(Velocity(velocity))
        .build()
        .spawn_in(world);
    game.handle(world, EntitySpawnEvent { entity });
}

/// System to add items to player inventories when the player comes near.
#[fecs::system]
pub fn item_collect(game: &mut Game, world: &mut World) {
    // run every 1/10 second
    if game.tick_count % (TPS / 10) != 0 {
        return;
    }

    let items_to_remove = Mutex::new(vec![]);
    let inventory_update_events = Mutex::new(vec![]);
    let item_collect_events = Mutex::new(vec![]);

    // For each player, check for nearby items and try collecting them
    // Safety: we only iterate over entities which are players,
    // and we only access item entities inside the loop. As such,
    // we will not have multiple mutable references to the same component.
    unsafe {
        <(Read<Position>, Write<Inventory>)>::query()
            .filter(component::<Player>())
            .filter(!component::<Dead>())
            .par_entities_for_each_unchecked(world.inner(), |(player, (pos, mut inventory))| {
                let inventory: &mut Inventory = &mut *inventory;

                let nearby_entities = nearby_entities(world, game, *pos, glm::vec3(1.0, 1.0, 1.0));
                let nearby_items = nearby_entities.iter().filter_map(|entity| {
                    world
                        .try_get::<CollectableAt>(*entity)
                        .map(|collectable_at| {
                            if collectable_at.0 <= game.time.world_age() {
                                Some(*entity)
                            } else {
                                None
                            }
                        })
                        .flatten()
                });

                for item in nearby_items {
                    debug_assert!(!world.has::<Player>(item));
                    // try to mark this item is collected
                    // (this ensures another thread has not collected it
                    // as well, which makes the mutable access below
                    // safe)
                    let is_removed = world.get::<IsRemoved>(item);

                    if !is_removed.0.compare_and_swap(false, true, Ordering::AcqRel) {
                        // we now have unique access to this item and its components.
                        let mut stack = world.get_mut_unchecked::<ItemStack>(item);

                        let (slots, stack_remaining) = inventory.collect_item(*stack);

                        let initial_remaining = stack.amount;

                        let event = InventoryUpdateEvent {
                            slots,
                            entity: player,
                        };
                        inventory_update_events.lock().push(event);

                        // update stack
                        if stack_remaining == 0 {
                            items_to_remove.lock().push(item);
                        } else {
                            stack.amount = stack_remaining;
                            world
                                .get_mut_unchecked::<EntityMetadata>(item)
                                .set(META_INDEX_ITEM_SLOT, Some(*stack));
                        }

                        item_collect_events.lock().push(ItemCollectEvent {
                            item,
                            collector: player,
                            amount: initial_remaining - stack_remaining,
                        });
                    }
                }
            });
    }

    // Trigger events + deferred entity deletes.
    for event in item_collect_events.into_inner() {
        game.handle(world, event);
    }

    for item in items_to_remove.into_inner() {
        game.despawn(item, world);
    }

    for event in inventory_update_events.into_inner() {
        game.handle(world, event);
    }

    // Reset `IsRemoved`.
    <Read<IsRemoved>>::query().for_each(world.inner(), |rem| rem.0.store(false, Ordering::Relaxed));
}

/// Returns an entity builder to create an item entity
/// with the given stack and collectable tick.
pub fn create(stack: ItemStack, collectable_at: u64) -> EntityBuilder {
    let meta = EntityMetadata::entity_base().with(META_INDEX_ITEM_SLOT, Some(stack));
    let collectable_at = CollectableAt(collectable_at);

    crate::base()
        .with(stack)
        .with(IsRemoved(AtomicBool::new(false)))
        .with(collectable_at)
        .with(SpawnPacketCreator(&create_spawn_packet))
        .with(ComponentSerializer(&serialize))
        .with(meta)
        .with(
            PhysicsBuilder::new()
                .bbox(0.25, 0.25, 0.25)
                .drag(0.98)
                .gravity(-0.04)
                .build(),
        )
}

fn create_spawn_packet(accessor: &EntityRef) -> Box<dyn Packet> {
    let position = *accessor.get::<Position>();
    let velocity = *accessor.get::<Velocity>();
    let entity_id = accessor.get::<NetworkId>().0;

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

fn serialize(game: &Game, accessor: &EntityRef) -> EntityData {
    let vel = accessor.get::<Velocity>().0;
    let item = accessor.get::<ItemStack>();
    let nbt = ItemNbt::from(*item);
    let nbt = if nbt == Default::default() {
        None
    } else {
        Some(nbt)
    };
    EntityData::Item(ItemEntityData {
        entity: BaseEntityData::new(*accessor.get::<Position>(), Vec3d::new(vel.x, vel.y, vel.z)),
        age: 0, // todo
        pickup_delay: (accessor.get::<CollectableAt>().0 as i64 - game.tick_count as i64).max(0)
            as i16,
        item: ItemData {
            count: item.amount as i8,
            item: item.ty.identifier().to_owned(),
            nbt,
        },
        health: 5, // todo
    })
}

fn load(data: EntityData) -> anyhow::Result<EntityBuilder> {
    match data {
        EntityData::Item(data) => {
            let pos = data.entity.read_position()?;
            let vel = data.entity.read_velocity()?;

            Item::from_identifier(&data.item.item)
                .ok_or_else(|| anyhow::anyhow!("invalid item {}", data.item.item))?;
            let stack = data.item.into();
            let collectable_at = data.pickup_delay;

            Ok(create(stack, collectable_at as u64)
                .with(pos)
                .with(Velocity(glm::vec3(vel.x, vel.y, vel.z))))
        }
        _ => panic!("attempted to use item::load to load a non-item"),
    }
}
