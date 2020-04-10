use crate::block::on_block_update_notify_adjacent;
use crate::broadcasters::{
    on_block_update_broadcast, on_chat_broadcast,
    on_entity_client_remove_update_last_known_positions, on_entity_despawn_broadcast_despawn,
    on_entity_send_send_equipment, on_entity_send_send_metadata,
    on_entity_send_update_last_known_positions, on_entity_spawn_send_to_clients,
    on_inventory_update_broadcast_equipment_update, on_inventory_update_send_set_slot,
    on_item_collect_broadcast, on_player_animation_broadcast_animation,
    on_player_join_send_existing_entities,
};
use crate::chat::{on_player_join_broadcast_join_message, ChatEvent};
use crate::chunk_entities::{
    on_chunk_cross_update_chunk_entities, on_entity_despawn_update_chunk_entities,
    on_entity_spawn_update_chunk_entities, ChunkEntities,
};
use crate::chunk_logic::{
    on_chunk_holder_release_unload_chunk, on_entity_despawn_remove_chunk_holder, ChunkHolders,
    ChunkUnloadQueue, ChunkWorkerHandle,
};
use crate::config::Config;
use crate::entity::falling_block::on_entity_land_remove_falling_block;
use crate::entity::item::{on_item_drop_spawn_item_entity, ItemCollectEvent, ItemDropEvent};
use crate::entity::Name;
use crate::io::{NetworkIoManager, NewClientInfo, ServerToWorkerMessage};
use crate::join::{on_chunk_send_join_player, on_player_join_send_join_game};
use crate::lighting::{
    on_block_update_notify_lighting_worker, on_chunk_load_notify_lighting_worker,
    on_chunk_unload_notify_lighting_worker, LightingWorkerHandle,
};
use crate::network::Network;
use crate::p_inventory::InventoryUpdateEvent;
use crate::physics::EntityPhysicsLandEvent;
use crate::player;
use crate::player::Player;
use crate::save::{
    on_chunk_load_queue_for_saving, on_chunk_unload_save_chunk, on_player_leave_save_data,
    SaveQueue,
};
use crate::task::RunningTasks;
use crate::time::{on_player_join_send_time, Time};
use crate::view::{
    on_chunk_cross_update_chunks, on_chunk_cross_update_entities, on_chunk_load_send_to_clients,
    on_player_join_trigger_chunk_cross, ChunksToSend,
};
use crate::weather::{
    on_weather_change_broadcast_weather, send_weather, Weather, WeatherChangeEvent,
};
use bumpalo::Bump;
use feather_blocks::Block;
use feather_core::level::LevelData;
use feather_core::world::ChunkMap;
use feather_core::{BlockPosition, ChunkPosition, ClientboundAnimation, Packet, Position};
use fecs::{Entity, IntoQuery, Read, World};
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::cell::{RefCell, RefMut};
use std::fmt::Display;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use thread_local::CachedThreadLocal;

/// Uber-resource storing almost all data needed to run the game.
///
/// This type includes the chunk map for accessing blocks,
/// time data, acceleration structures, and practically all
/// game state, except entities, which are stored in the `World`.
pub struct Game {
    /// The IO handle.
    pub io_handle: NetworkIoManager,
    /// The server configuration.
    pub config: Arc<Config>,
    /// The server tick count, measured in ticks
    /// since program startup.
    pub tick_count: u64,
    /// The server player count.
    ///
    /// (This value is stored in an `Arc` so it can
    /// be shared with the status ping threads.)
    pub player_count: Arc<AtomicU32>,
    /// Information about the world, such as spawn position
    /// and world type.
    pub level: LevelData,
    /// The chunk map.
    pub chunk_map: ChunkMap,
    /// Bump allocator. Reset every tick.
    pub(super) bump: CachedThreadLocal<Bump>,
    /// Chunk worker handle used for communication with
    /// the chunk worker.
    pub chunk_worker_handle: ChunkWorkerHandle,
    /// Queue of chunks to be unloaded.
    pub chunk_unload_queue: ChunkUnloadQueue,
    pub chunk_holders: ChunkHolders,
    pub chunks_to_send: ChunksToSend,
    pub chunk_entities: ChunkEntities,
    pub(super) rng: CachedThreadLocal<RefCell<XorShiftRng>>,
    pub time: Time,
    pub save_queue: SaveQueue,
    pub lighting_worker_handle: LightingWorkerHandle,
    pub running_tasks: RunningTasks,
}

impl Game {
    /// Retrieves the block at the given position,
    /// or `None` if the block's chunk is not loaded.
    pub fn block_at(&self, pos: BlockPosition) -> Option<Block> {
        self.chunk_map.block_at(pos)
    }

    /// Sets the block at the given position.
    ///
    /// If the block's chunk's is not loaded, returns `false`;
    /// otherwise, returns `true`.
    pub fn set_block_at(&mut self, world: &mut World, pos: BlockPosition, block: Block) -> bool {
        let old_block = match self.block_at(pos) {
            Some(block) => block,
            None => return false,
        };

        let result = self.chunk_map.set_block_at(pos, block);

        self.on_block_update(world, pos, old_block, block);

        result
    }

    /// Despawns an entity. This should be used instead of `World::despawn`
    /// as it properly handles events.
    pub fn despawn(&mut self, entity: Entity, world: &mut World) {
        self.on_entity_despawn(world, entity);
        world.despawn(entity);
    }

    /// Spawns a player with the given `PlayerInfo`.
    pub fn spawn_player(&mut self, info: NewClientInfo, world: &mut World) {
        player::create(self, world, info);
    }

    /// Returns a bump allocator.
    pub fn bump(&self) -> &Bump {
        self.bump.get_or_default()
    }

    /// Returns a random number generator.
    pub fn rng(&self) -> RefMut<impl Rng> {
        self.rng
            .get_or(|| RefCell::new(XorShiftRng::from_entropy()))
            .borrow_mut()
    }

    /// Disconnects a player.
    pub fn disconnect(&mut self, player: Entity, world: &mut World, reason: impl Display) {
        let network = world.get::<Network>(player);

        let name = world.get::<Name>(player);
        info!("{} disconnected: {}", name.0, reason);

        let _ = network.tx.unbounded_send(ServerToWorkerMessage::Disconnect);

        drop(name);
        drop(network);

        self.despawn(player, world);
    }

    /* BROADCAST FUNCTIONS */
    /// Broadcasts a packet to all online players.
    pub fn broadcast_global(&self, world: &World, packet: impl Packet, neq: Option<Entity>) {
        self.broadcast_global_boxed(world, Box::new(packet), neq);
    }

    /// Broadcasts a boxed packet to all online players.
    pub fn broadcast_global_boxed(
        &self,
        world: &World,
        packet: Box<dyn Packet>,
        neq: Option<Entity>,
    ) {
        for (entity, network) in <Read<Network>>::query().iter_entities(world.inner()) {
            if neq.map(|neq| neq == entity).unwrap_or(false) {
                continue;
            }

            network.send_boxed(packet.box_clone());
        }
    }

    /// Broadcasts a packet to all players able to see a given chunk.
    pub fn broadcast_chunk_update(
        &self,
        world: &World,
        packet: impl Packet,
        chunk: ChunkPosition,
        neq: Option<Entity>,
    ) {
        self.broadcast_chunk_update_boxed(world, Box::new(packet), chunk, neq);
    }

    /// Broadcasts a boxed packet to all players able to see a given chunk.
    pub fn broadcast_chunk_update_boxed(
        &self,
        world: &World,
        packet: Box<dyn Packet>,
        chunk: ChunkPosition,
        neq: Option<Entity>,
    ) {
        // we can use the chunk holders structure to accelerate this
        for entity in self.chunk_holders.holders_for(chunk) {
            if neq.map(|neq| neq == *entity).unwrap_or(false) {
                continue;
            }

            if let Some(network) = world.try_get::<Network>(*entity) {
                network.send_boxed(packet.box_clone());
            }
        }
    }

    /// Broadcasts a packet to all players able to see a given entity.
    pub fn broadcast_entity_update(
        &self,
        world: &World,
        packet: impl Packet,
        entity: Entity,
        neq: Option<Entity>,
    ) {
        self.broadcast_entity_update_boxed(world, Box::new(packet), entity, neq);
    }

    /// Broadcasts a boxed packet to all players able to see a given entity.
    pub fn broadcast_entity_update_boxed(
        &self,
        world: &World,
        packet: Box<dyn Packet>,
        entity: Entity,
        neq: Option<Entity>,
    ) {
        // Send the packet to all players who have a hold on the entity's chunk.
        let entity_chunk = world.get::<Position>(entity).chunk();
        self.broadcast_chunk_update_boxed(world, packet, entity_chunk, neq);
    }

    /* EVENT HANDLERS */
    /// Called when a block is updated.
    pub fn on_block_update(
        &mut self,
        world: &mut World,
        pos: BlockPosition,
        old: Block,
        new: Block,
    ) {
        on_block_update_notify_adjacent(self, world, pos);
        on_block_update_broadcast(self, world, pos, new);
        on_block_update_notify_lighting_worker(self, pos, old, new);
    }

    /// Called when an entity is despawned/removed.
    ///
    /// Note that this is called __before__ the entity is deleted from the world.
    /// As such, components of the entity can still be accessed.
    pub fn on_entity_despawn(&mut self, world: &mut World, entity: Entity) {
        on_entity_despawn_remove_chunk_holder(self, world, entity);
        on_entity_despawn_update_chunk_entities(self, world, entity);
        on_entity_despawn_broadcast_despawn(self, world, entity);
        if world.try_get::<Player>(entity).is_some() {
            self.on_player_leave(world, entity);
        }
    }

    /// Called when an entity of any type is spawned/created.
    ///
    /// This function is only called for "normal" entities, i.e.
    /// those which Minecraft normally considers entities. Auxiliary
    /// entities used by Feather, such as the block notify entity
    /// (see `crate::block`) are not included in this function.
    pub fn on_entity_spawn(&mut self, world: &mut World, entity: Entity) {
        on_entity_spawn_update_chunk_entities(self, world, entity);
        on_entity_spawn_send_to_clients(self, world, entity);
    }

    /// Called when an entity is spawned on a client.
    pub fn on_entity_send(&self, world: &mut World, entity: Entity, client: Entity) {
        on_entity_send_update_last_known_positions(world, entity, client);
        on_entity_send_send_equipment(world, entity, client);
        on_entity_send_send_metadata(world, entity, client);
    }

    /// Called when an entity is removed on a client (Destroy Entities packet)
    pub fn on_entity_client_remove(&mut self, world: &mut World, entity: Entity, client: Entity) {
        on_entity_client_remove_update_last_known_positions(world, entity, client);
    }

    /// Called when a player joins.
    pub fn on_player_join(&mut self, world: &mut World, player: Entity) {
        self.player_count.fetch_add(1, Ordering::Relaxed);
        on_player_join_send_join_game(self, world, player);
        on_player_join_send_existing_entities(world, player);
        on_player_join_send_time(self, world, player);
        on_player_join_trigger_chunk_cross(self, world, player);
        send_weather(world, player, self.weather());
        on_player_join_broadcast_join_message(self, world, player);
    }

    /// Called when a player leaves.
    ///
    /// As with `on_entity_despawn`, this function is called __before__
    /// `player` is removed from the world.
    pub fn on_player_leave(&mut self, world: &mut World, player: Entity) {
        self.player_count.fetch_sub(1, Ordering::Relaxed);
        on_player_leave_save_data(self, world, player);
    }

    /// Called when a chunk loads successfully.
    pub fn on_chunk_load(&mut self, world: &mut World, chunk: ChunkPosition) {
        on_chunk_load_notify_lighting_worker(self, chunk);
        on_chunk_load_send_to_clients(self, world, chunk);
        on_chunk_load_queue_for_saving(self, chunk);
    }

    /// Called when a chunk unloads.
    ///
    /// This is called _before_ the chunk is removed from the chunk map.
    pub fn on_chunk_unload(&mut self, world: &mut World, chunk: ChunkPosition) {
        on_chunk_unload_save_chunk(self, world, chunk);
        on_chunk_unload_notify_lighting_worker(self, chunk);
    }

    /// Called when a chunk fails to load.
    pub fn on_chunk_load_fail(&mut self, _world: &mut World, _chunk: ChunkPosition) {}

    /// Called when a chunk holder is released.
    pub fn on_chunk_holder_release(&mut self, chunk: ChunkPosition, _holder: Entity) {
        on_chunk_holder_release_unload_chunk(self, chunk);
    }

    /// Called when an entity crosses into a new chunk.
    pub fn on_chunk_cross(
        &mut self,
        world: &mut World,
        entity: Entity,
        old: Option<ChunkPosition>,
        new: ChunkPosition,
    ) {
        on_chunk_cross_update_chunks(self, world, entity, old, new);
        on_chunk_cross_update_chunk_entities(self, entity, old, new);
        on_chunk_cross_update_entities(self, world, entity, old, new);
    }

    /// Called when a chunk is sent to a client.
    pub fn on_chunk_send(&self, world: &mut World, chunk: ChunkPosition, player: Entity) {
        on_chunk_send_join_player(self, world, chunk, player);
    }

    /// Called when a player's inventory is updated.
    pub fn on_inventory_update(&mut self, world: &mut World, event: InventoryUpdateEvent) {
        on_inventory_update_send_set_slot(world, &event);
        on_inventory_update_broadcast_equipment_update(self, world, &event);
    }

    /// Called when a player causes an animation.
    pub fn on_player_animation(
        &mut self,
        world: &mut World,
        player: Entity,
        animation: ClientboundAnimation,
    ) {
        on_player_animation_broadcast_animation(self, world, player, animation);
    }

    /// Called when an item is dropped by a player.
    pub fn on_item_drop(&mut self, world: &mut World, event: ItemDropEvent) {
        on_item_drop_spawn_item_entity(self, world, &event);
    }

    /// Called when an entity collects an item entity.
    pub fn on_item_collect(&mut self, world: &mut World, event: ItemCollectEvent) {
        on_item_collect_broadcast(self, world, &event);
    }

    /// Called when weather changes
    pub fn on_weather_change(&mut self, world: &mut World, event: &mut WeatherChangeEvent) {
        on_weather_change_broadcast_weather(self, world, event.to);
    }

    /// Returns the current state of the weather
    pub fn weather(&self) -> Weather {
        crate::weather::get_weather(&self)
    }

    /// Sets the weather for a given duration
    pub fn set_weather(&mut self, weather: Weather, duration: i32) -> Weather {
        crate::weather::set_weather(self, weather, duration)
    }

    /// Called when a chat message is broadcasted.
    pub fn on_chat(&mut self, world: &mut World, event: ChatEvent) {
        on_chat_broadcast(self, world, &event);
    }

    /// Called when an entity lands on the ground.
    pub fn on_entity_land(&mut self, world: &mut World, event: EntityPhysicsLandEvent) {
        on_entity_land_remove_falling_block(self, world, &event);
    }
}

#[system]
pub fn increment_tick_count(game: &mut Game) {
    game.tick_count += 1;
}

#[system]
pub fn reset_bump_allocators(game: &mut Game) {
    game.bump.iter_mut().for_each(Bump::reset);
}
