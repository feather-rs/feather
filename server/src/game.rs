use crate::chunk_entities::{
    on_chunk_cross_update_chunk_entities, on_entity_despawn_update_chunk_entities, ChunkEntities,
};
use crate::chunk_logic::{ChunkHolders, ChunkUnloadQueue, ChunkWorkerHandle};
use crate::config::Config;
use crate::io::{NetworkIoManager, NewClientInfo};
use crate::join::{on_chunk_send_join_player, on_player_join_send_join_game};
use crate::view::{
    on_chunk_cross_update_chunks, on_chunk_load_send_to_clients,
    on_player_join_trigger_chunk_cross, ChunksToSend,
};
use crate::{chunk_logic, player};
use bumpalo::Bump;
use feather_blocks::Block;
use feather_core::level::LevelData;
use feather_core::world::ChunkMap;
use feather_core::{BlockPosition, ChunkPosition};
use fecs::{Entity, World};
use std::sync::atomic::AtomicU32;
use std::sync::Arc;

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
    pub bump: Bump,
    /// Chunk worker handle used for communication with
    /// the chunk worker.
    pub chunk_worker_handle: ChunkWorkerHandle,
    /// Queue of chunks to be unloaded.
    pub chunk_unload_queue: ChunkUnloadQueue,
    pub chunk_holders: ChunkHolders,
    pub chunks_to_send: ChunksToSend,
    pub chunk_entities: ChunkEntities,
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

        self.on_block_update(world, pos, old_block, block);

        self.chunk_map.set_block_at(pos, block)
    }

    /// Despawns an entity. This should be used instead of `World::despawn`
    /// as it properly handles events.
    pub fn despawn(&mut self, entity: Entity, world: &mut World) {
        self.on_entity_despawn(world, entity);
        world.despawn(entity);
    }

    /// Spawns a player with the given `PlayerInfo`.
    pub fn spawn_player(&mut self, info: NewClientInfo, world: &mut World) {
        let entity = player::create(world, info);
        self.on_player_join(world, entity);
    }

    /* EVENT HANDLERS */
    /// Called when a block is updated.
    pub fn on_block_update(
        &mut self,
        _world: &mut World,
        _pos: BlockPosition,
        _old: Block,
        _new: Block,
    ) {
    }

    /// Called when an entity is despawned/removed.
    ///
    /// Note that this is called __before__ the entity is deleted from the world.
    /// As such, components of the entity can still be accessed.
    pub fn on_entity_despawn(&mut self, world: &mut World, entity: Entity) {
        chunk_logic::on_entity_despawn_remove_chunk_holder(self, world, entity);
        on_entity_despawn_update_chunk_entities(self, world, entity);
    }

    /// Called when a player joins.
    pub fn on_player_join(&mut self, world: &mut World, player: Entity) {
        on_player_join_trigger_chunk_cross(self, world, player);
        on_player_join_send_join_game(self, world, player);
    }

    /// Called when a chunk loads successfully.
    pub fn on_chunk_load(&mut self, world: &mut World, chunk: ChunkPosition) {
        on_chunk_load_send_to_clients(self, world, chunk);
    }

    /// Called when a chunk fails to load.
    pub fn on_chunk_load_fail(&mut self, _world: &mut World, _chunk: ChunkPosition) {}

    /// Called when a chunk holder is released.
    pub fn on_chunk_holder_release(&mut self, chunk: ChunkPosition, _holder: Entity) {
        chunk_logic::on_chunk_holder_release_unload_chunk(self, chunk);
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
    }

    /// Called when a chunk is sent to a client.
    pub fn on_chunk_send(&self, world: &mut World, chunk: ChunkPosition, player: Entity) {
        on_chunk_send_join_player(self, world, chunk, player);
    }
}

#[system]
pub fn increment_tick_count(game: &mut Game) {
    game.tick_count += 1;
}
