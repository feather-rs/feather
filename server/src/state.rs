use crate::block::{BlockUpdateCause, BlockUpdateEvent};
use crate::broadcasters::movement::LastKnownPositions;
use crate::chunk_entities::ChunkEntities;
use crate::chunk_logic::ChunkHolders;
use crate::config::Config;
use crate::entity::EntitySendEvent;
use crate::lazy::{EntityBuilder, Lazy};
use crate::network::Network;
use feather_blocks::Block;
use feather_core::level::LevelData;
use feather_core::world::ChunkMap;
use feather_core::{BlockPosition, Chunk, ChunkPosition, Packet, Position};
use legion::borrow::AtomicRefCell;
use legion::entity::Entity;
use legion::query::{IntoQuery, Read};
use legion::storage::ComponentTypeId;
use legion::world::World;
use parking_lot::RwLockReadGuard;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use tonks::{
    MacroData, ResourceId, Resources, Scheduler, SystemCtx, SystemData, SystemDataOutput, Trigger,
};

/// Resource used internally by `State`.
#[derive(Resource)]
pub struct StateInner {
    pub config: Arc<Config>,
    pub chunk_map: ChunkMap,
    pub level: LevelData,
    pub chunk_entities: ChunkEntities,

    lazy: Lazy,
}

impl StateInner {
    pub fn new(config: Arc<Config>, chunk_map: ChunkMap, level: LevelData) -> Self {
        Self {
            config,
            chunk_map,
            level,
            chunk_entities: ChunkEntities::default(),
            lazy: Lazy::default(),
        }
    }

    /// See `Lazy::flush()`.
    pub fn flush(&self, world: &mut World, scheduler: &mut Scheduler) {
        self.lazy.flush(world, scheduler);
    }
}

/// The state of the server.
///
/// This state wraps numerous commonly-used resources,
/// including the chunk map (block access), config, and
/// various cached data structures, among others.
///
/// Systems should never require mutable access to the
/// state; it is designed for read-only use. (The chunk
/// map uses `RwLock` internally, so write access isn't
/// needed to update blocks.)
///
/// # Internal details
/// A custom `tonks::SystemData` implementation
/// is utilized so that functions on `State` can automatically
/// trigger events. For example, the methods which update
/// blocks automatically trigger `BlockUpdateEvent`s.
///
/// An implication of this implementation is that `State` itself
/// is not a resource; however, `StateInner` is.
pub struct State {
    inner: *mut StateInner,
    trigger: AtomicRefCell<Trigger<BlockUpdateEvent>>, // TODO: optimize
}

unsafe impl Send for State {}
unsafe impl Sync for State {}

impl<'a> SystemData<'a> for State {
    type Output = &'a Self;

    unsafe fn load_from_resources(
        resources: &mut Resources,
        ctx: SystemCtx,
        world: &World,
    ) -> Self {
        let inner = resources
            .get_mut_unchecked::<StateInner>(tonks::resource_id_for::<StateInner>())
            as *mut StateInner;
        let trigger = Trigger::load_from_resources(resources, ctx, world);

        Self {
            inner,
            trigger: AtomicRefCell::new(trigger),
        }
    }

    fn resource_reads() -> Vec<ResourceId> {
        vec![tonks::resource_id_for::<StateInner>()]
    }

    fn resource_writes() -> Vec<ResourceId> {
        vec![]
    }

    fn component_reads() -> Vec<ComponentTypeId> {
        vec![]
    }

    fn component_writes() -> Vec<ComponentTypeId> {
        vec![]
    }

    fn before_execution(&'a mut self) -> Self::Output {
        self
    }

    fn after_execution(&mut self) {
        self.trigger.get_mut().after_execution()
    }
}

impl<'a> SystemDataOutput<'a> for &'a State {
    type SystemData = State;
}

impl MacroData for &'static State {
    type SystemData = State;
}

impl State {
    /// See `Lazy::exec()`.
    pub fn exec(&self, f: impl FnOnce(&mut World) + Send + 'static) {
        self.lazy.exec(f)
    }

    /// See `Lazy::exec_with_scheduler()`.
    pub fn exec_with_scheduler(&self, f: impl FnOnce(&mut World, &mut Scheduler) + Send + 'static) {
        self.lazy.exec_with_scheduler(f)
    }

    /// See `Lazy::create_entity()`.
    pub fn create_entity(&self) -> EntityBuilder {
        self.lazy.create_entity()
    }

    /// Lazily broadcasts a packet to all clients able to see the given entity.
    ///
    /// The packet will not be sent to `neq`.
    pub fn broadcast_entity_update<P: Packet + Clone>(
        &self,
        entity: Entity,
        packet: P,
        neq: Option<Entity>,
    ) {
        self.exec_with_scheduler(move |world, scheduler| {
            // Use ChunkHolders to determine which players have a hold on the entity's
            // chunk, which would allow them to see the entity.
            let chunk_holders = scheduler.resources().get::<ChunkHolders>();

            if let Some(position) = world.get_component::<Position>(entity) {
                let holders = chunk_holders.holders_for(position.chunk_pos());

                holders.map(|entities| {
                    for entity in entities {
                        if let Some(network) = world.get_component::<Network>(*entity) {
                            if neq.map_or(true, |neq| *entity != neq) {
                                network.send(packet.clone());
                            }
                        }
                    }
                });
            }
        });
    }

    /// Lazily broadcasts a boxed packet to all clients able to see the given entity.
    ///
    /// The packet will not be sent to `neq`.
    pub fn broadcast_entity_update_boxed(
        &self,
        entity: Entity,
        packet: Box<dyn Packet>,
        neq: Option<Entity>,
    ) {
        self.exec_with_scheduler(move |world, scheduler| {
            // Use ChunkHolders to determine which players have a hold on the entity's
            // chunk, which would allow them to see the entity.
            let chunk_holders = scheduler.resources().get::<ChunkHolders>();

            if let Some(position) = world.get_component::<Position>(entity) {
                let holders = chunk_holders.holders_for(position.chunk_pos());

                holders.map(|entities| {
                    for entity in entities {
                        if let Some(network) = world.get_component::<Network>(*entity) {
                            if neq.map_or(true, |neq| *entity != neq) {
                                network.send_boxed(packet.box_clone());
                            }
                        }
                    }
                });
            }
        });
    }

    /// Lazily broadcasts a packet to all players able to see the given chunk.
    ///
    /// The packet will not be sent to `neq`.
    pub fn broadcast_chunk_update(
        &self,
        chunk: ChunkPosition,
        packet: impl Packet + Clone,
        neq: Option<Entity>,
    ) {
        self.exec_with_scheduler(move |world, scheduler| {
            // Use ChunkHolders to determine which players have a hold on the
            // chunk, which would allow them to see the entity.
            let chunk_holders = scheduler.resources().get::<ChunkHolders>();

            let holders = chunk_holders.holders_for(chunk);

            holders.map(|entities| {
                for entity in entities {
                    if let Some(network) = world.get_component::<Network>(*entity) {
                        if neq.map_or(true, |neq| *entity != neq) {
                            network.send(packet.clone());
                        }
                    }
                }
            });
        });
    }

    /// Lazily broadcasts a packet to all clients.
    pub fn broadcast_global<P: Packet + Clone>(&self, packet: P, neq: Option<Entity>) {
        self.exec(move |world| {
            // Standard Legion queries! How rare.
            let query = <Read<Network>>::query();

            query.par_entities_for_each(world, |(entity, network)| {
                if neq.map_or(true, |neq| entity != neq) {
                    network.send(packet.clone());
                }
            });
        });
    }

    /// Lazily broadcasts a boxed packet to all clients.
    pub fn broadcast_global_boxed(&self, packet: Box<dyn Packet>, neq: Option<Entity>) {
        self.exec(move |world| {
            // Standard Legion queries! How rare.
            let query = <Read<Network>>::query();

            query.par_entities_for_each(world, |(entity, network)| {
                if neq.map_or(true, |neq| entity != neq) {
                    network.send_boxed(packet.box_clone());
                }
            });
        });
    }

    /// Retrieves the block at the given position,
    /// or `None` if the block's chunk is not loaded.
    pub fn block_at(&self, pos: BlockPosition) -> Option<Block> {
        self.chunk_map.block_at(pos)
    }

    /// Sets the block at the given position.
    ///
    /// If the block's chunk's is not loaded, returns `false`;
    /// otherwise, returns `true`.
    pub fn set_block_at(&self, pos: BlockPosition, block: Block, cause: BlockUpdateCause) -> bool {
        let old_block = match self.block_at(pos) {
            Some(block) => block,
            None => return false,
        };

        let event = BlockUpdateEvent {
            cause,
            pos,
            old_block,
            new_block: block,
        };
        self.trigger.get_mut().trigger(event);

        self.chunk_map.set_block_at(pos, block)
    }

    /// Retrieves a reference to the chunk at the given position,
    /// or `None` if it not loaded.
    pub fn chunk_at(&self, pos: ChunkPosition) -> Option<RwLockReadGuard<Chunk>> {
        self.chunk_map.chunk_at(pos)
    }

    /// Lazily inserts the given chunk into the chunk map.
    pub fn lazy_insert_chunk(&self, chunk: Chunk) {
        self.lazy.exec_with_scheduler(move |_, scheduler| unsafe {
            scheduler
                .resources()
                .get_mut_unchecked::<StateInner>(tonks::resource_id_for::<StateInner>())
                .chunk_map
                .insert(chunk);
        });
    }

    /// Lazily removes the given chunk from the chunk map.
    pub fn lazy_remove_chunk(&self, pos: ChunkPosition) {
        self.lazy
            .exec_with_scheduler(move |_: &mut World, scheduler: &mut Scheduler| unsafe {
                scheduler
                    .resources()
                    .get_mut_unchecked::<StateInner>(tonks::resource_id_for::<StateInner>())
                    .chunk_map
                    .remove(pos);
            });
    }

    /// Registers that an entity was sent to a player, updating some
    /// data structures, such as LastKnownPositions.
    pub fn register_entity_send(&self, entity: Entity, to: Entity) {
        self.exec_with_scheduler(move |world, scheduler| {
            let pos = *world.get_component(entity).unwrap();
            if let Some(mut positions) = world.get_component_mut::<LastKnownPositions>(to) {
                positions.0.insert(entity, pos);
            }

            scheduler.trigger(EntitySendEvent { entity, to });
        });
    }

    /// The opposite of `register_entity_send`.
    pub fn register_entity_unload(&self, entity: Entity, on: Entity) {
        self.exec(move |world| {
            if let Some(mut positions) = world.get_component_mut::<LastKnownPositions>(on) {
                positions.0.remove(&entity);
            }
        })
    }
}

impl Deref for State {
    type Target = StateInner;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.inner }
    }
}

impl DerefMut for State {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.inner }
    }
}
