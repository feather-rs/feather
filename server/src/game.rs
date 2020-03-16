use crate::config::Config;
use crate::io::{NetworkIoManager, NewClientInfo};
use crate::player;
use bumpalo::Bump;
use feather_blocks::Block;
use feather_core::level::LevelData;
use feather_core::world::ChunkMap;
use feather_core::BlockPosition;
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
        world.despawn(entity);
        self.on_despawn(world, entity);
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
    pub fn on_despawn(&mut self, _world: &mut World, _entity: Entity) {}

    /// Called when a player joins.
    pub fn on_player_join(&mut self, _world: &mut World, _player: Entity) {}
}
