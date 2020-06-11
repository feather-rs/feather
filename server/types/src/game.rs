use crate::{BlockUpdateCause, Network, ServerToWorkerMessage};
use crate::{
    BlockUpdateEvent, CanRespawn, Dead, EntityDeathEvent, EntityDespawnEvent, Health,
    HealthUpdateEvent, Name, PlayerLeaveEvent,
};
use ahash::AHashMap;
use bumpalo::Bump;
use feather_core::anvil::level::LevelData;
use feather_core::blocks::BlockId;
use feather_core::chunk_map::ChunkMap;
use feather_core::network::Packet;
use feather_core::util::{BlockPosition, ChunkPosition, Position};
use feather_server_config::Config;
use fecs::{Entity, Event, EventHandlers, IntoQuery, OwnedResources, Read, RefResources, World};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use smallvec::SmallVec;
use std::cell::{RefCell, RefMut};
use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use thread_local::CachedThreadLocal;

/// Resources which can be _shared_ between threads.
/// These only require immutable access.
pub struct Shared {
    /// The server configuration.
    pub config: Arc<Config>,
    /// General-purpose, non-cryptographic random number generator
    pub rng: CachedThreadLocal<RefCell<SmallRng>>,
    /// The server player count.
    pub player_count: Arc<AtomicU32>, // fixme: double Arc
}

/// The `Game` resource, which acts as a central bus to bind together
/// the feather-server-* crates. Resources which are accessed frequently,
/// such as the chunk map, are stored in here.
pub struct Game {
    pub chunk_map: ChunkMap,
    /// Number of ticks since the program started. Can be used
    /// to make a system which only runs at a fixed interval.
    pub tick_count: u64,
    /// Stores entities which have a hold on chunks,
    /// preventing the chunk from being unloaded.
    pub chunk_holders: ChunkHolders,
    /// Block entity map. Each `BlockPosition` may have a block
    /// entity associated with it.
    pub block_entities: AHashMap<BlockPosition, Entity>,
    /// The level data.
    pub level: LevelData,
    /// Associates chunks with the entities that reside in them. Used
    /// as an acceleration structure for spacial lookups.
    pub chunk_entities: ChunkEntities,
    /// World time, in the Minecraft way.
    pub time: Time,
    /// The event handler map.
    pub event_handlers: Arc<EventHandlers>,
    /// Resources other than `Game`, used to run event handlers.
    pub resources: Arc<OwnedResources>,
    /// Shared bump allocator, reset each tick.
    pub bump: CachedThreadLocal<Bump>,
    /// Values which can be shared between threads.
    pub shared: Arc<Shared>,
}

impl Deref for Game {
    type Target = Shared;

    fn deref(&self) -> &Self::Target {
        &self.shared
    }
}

impl Game {
    /// Handles an event or message. All handlers
    /// for the given event will be run.
    pub fn handle(&mut self, world: &mut World, event: impl Event) {
        // TODO: optimize this by avoiding Rc clone.
        let resources = Arc::clone(&self.resources);
        let event_handlers = Arc::clone(&self.event_handlers);
        let resources = RefResources::new(Arc::as_ref(&resources), (self,));
        event_handlers.trigger(&resources, world, event);
    }

    /// Retrieves the block at the given position,
    /// or `None` if the block's chunk is not loaded.
    pub fn block_at(&self, pos: BlockPosition) -> Option<BlockId> {
        self.chunk_map.block_at(pos)
    }

    /// Sets the block at the given position.
    ///
    /// If the block's chunk's is not loaded, returns `false`;
    /// otherwise, returns `true`.
    pub fn set_block_at(
        &mut self,
        world: &mut World,
        pos: BlockPosition,
        block: BlockId,
        cause: BlockUpdateCause,
    ) -> bool {
        let old = match self.block_at(pos) {
            Some(block) => block,
            None => return false,
        };

        let result = self.chunk_map.set_block_at(pos, block);

        self.handle(
            world,
            BlockUpdateEvent {
                pos,
                old,
                new: block,
                cause,
            },
        );

        result
    }

    /// Returns a bump allocator.
    pub fn bump(&self) -> &Bump {
        self.bump.get_or_default()
    }

    /// Returns a random number generator.
    pub fn rng(&self) -> RefMut<impl Rng> {
        self.rng
            .get_or(|| RefCell::new(SmallRng::from_entropy()))
            .borrow_mut()
    }

    /// Despawns an entity. This should be used instead of `World::despawn`
    /// as it properly handles events.
    pub fn despawn(&mut self, entity: Entity, world: &mut World) {
        self.handle(world, EntityDespawnEvent { entity });
        world.despawn(entity);
    }

    /// Disconnects a player.
    pub fn disconnect(&mut self, player: Entity, world: &mut World, reason: impl Display) {
        let network = world.get::<Network>(player);

        let name = world.get::<Name>(player);
        log::info!("{} disconnected: {}", name.0, reason);

        let _ = network.tx.send(ServerToWorkerMessage::Disconnect);

        drop(name);
        drop(network);

        self.player_count.fetch_sub(1, Ordering::AcqRel);

        self.handle(world, PlayerLeaveEvent { player });
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

    /// Applies damage to the given entity. Handles all logic,
    /// including killing the entity if its health drops below 1.
    pub fn damage(&mut self, entity: Entity, damage: u32, world: &mut World) {
        if world.has::<Dead>(entity) {
            return;
        }

        let (should_kill, old_health, new_health) =
            if let Some(mut health) = world.try_get_mut::<Health>(entity) {
                let old_health = health.0;
                let should_kill = match health.0.checked_sub(damage) {
                    Some(0) => true,
                    None => {
                        // below 0
                        health.0 = 0;
                        true
                    }
                    Some(new_health) => {
                        health.0 = new_health;
                        false
                    }
                };
                let new_health = health.0;
                (should_kill, Some(old_health), new_health)
            } else {
                (false, None, 0)
            };

        if let Some(old_health) = old_health {
            self.handle(
                world,
                HealthUpdateEvent {
                    old: old_health,
                    new: new_health,
                    entity,
                },
            );
        }

        if should_kill {
            self.kill(entity, world);
        }
    }

    /// Kills an entity.
    pub fn kill(&mut self, entity: Entity, world: &mut World) {
        // Don't kill if already on respawn screen
        if world.has::<Dead>(entity) {
            return;
        }

        self.handle(world, EntityDeathEvent { entity });
        if !world.has::<CanRespawn>(entity) {
            self.despawn(entity, world);
        }
    }
}

/// The chunk holder map contains a mapping
/// of chunk positions to any number of entities, called "holders."
/// When a chunk position has no holders, it will be queued
/// for unloading.
///
/// In addition, the chunk holders map can be used to select
/// which players to broadcast an entity movement to: a player
/// who has a chunk hold on the entity's chunk would be able to see
/// the movement, while other players would be outside of the view
/// distance. This technique allows for higher performance and
/// avoids constant nearby entity queries.
#[derive(Default, Clone, Debug)]
pub struct ChunkHolders {
    pub inner: AHashMap<ChunkPosition, SmallVec<[Entity; 4]>>,
}

impl ChunkHolders {
    pub fn holders_for(&self, chunk: ChunkPosition) -> &[Entity] {
        self.inner
            .get(&chunk)
            .map(SmallVec::as_slice)
            .unwrap_or(&[])
    }

    pub fn chunk_has_holders(&self, chunk: ChunkPosition) -> bool {
        let holders = self.holders_for(chunk);

        !holders.is_empty()
    }

    pub fn insert_holder(&mut self, chunk: ChunkPosition, holder: Entity) {
        self.inner.entry(chunk).or_default().push(holder)
    }
}

/// Stores which entities belong to every given chunk.
///
/// This data structure can be used to accelerate certain
/// operations, such as querying for entities
/// within some distance of a position. In addition,
/// it can be used to send all entities in a chunk
/// to a player.
///
/// Do note that the information in this structure is not necessarily up to date,
/// although a best effort is made to update the data.
#[derive(Default)]
pub struct ChunkEntities(pub AHashMap<ChunkPosition, SmallVec<[Entity; 4]>>);

impl ChunkEntities {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a slice of entities in the given chunk.
    pub fn entities_in_chunk(&self, chunk: ChunkPosition) -> &[Entity] {
        self.0.get(&chunk).map(|vec| vec.as_slice()).unwrap_or(&[])
    }
}

/// The current time of the world.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Time(pub u64);

impl Deref for Time {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Time {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Time {
    /// Returns the time of day. This is calculated
    /// as `time.0 % 24_000`.
    pub fn time_of_day(self) -> u64 {
        self.0 % 24_000
    }

    /// Returns the age of the world in ticks. Equivalent to `time.0`.
    pub fn world_age(self) -> u64 {
        self.0
    }
}

#[fecs::system]
pub fn reset_bump_allocators(game: &mut Game) {
    game.bump.iter_mut().for_each(Bump::reset);
}

#[fecs::system]
pub fn increment_tick_count(game: &mut Game) {
    game.tick_count += 1;
}
