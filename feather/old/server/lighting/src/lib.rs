//! An implementation of lighting, primarily based on 3D flood fill
//! algorithms.
//!
//! # Structure
//! Lighting is done on a separate _lighting worker thread_ which
//! stores its own copy of the chunk map. The server notifies
//! it when chunks are loaded and unloaded, and it can
//! request that it handle a lighting update, either for
//! an entire chunk or for a single block update. Since the lighting
//! worker has clones of the `Arc`s in which chunks are held, any
//! updates it makes to light data are visible to the server thread.
//!
//! # Algorithms: block light
//! For block light calculation, we define four types of block
//! updates for which to perform lighting:
//!
//! * Creation of a light-emitting block. We simply propagate
//! the light update using flood fill.
//!
//! * Removal of a light-emitting block. We first perform flood fill
//! and set any blocks which were previously affected by this block's
//! light to 0. Then, we recalculate lighting for light sources within
//! a range of 30 blocks based on algorithm #1.
//!
//! * Creation of an opaque, non-emitting block. We first set the created
//! block to air temporarily. We then query for nearby lights
//! within a range of 15 (the maximum distance travelled by light) and perform
//! algorithm #2 on them. Finally, we set the created block back to the correct
//! value and perform algorithm #1 on all lights.
//!
//! * Removal of an opaque, non-emitting block. In this case,
//! we set the new air block's light to the highest value of an
//! adjacent block minus 1. We then perform algorithm #1 on this new block.
//!
//! Each algorithm is implemented in a separate function, and `LightingSystem`
//! determines which to use based on the values of the block update event.
//!
//! If we are recalculating light for an entire chunk, e.g. when a chunk is generated,
//! we first zero out light, then find all light sources in the chunk and perform
//! algorithm #1 on them as if they had just been placed.

extern crate nalgebra_glm as glm;

#[cfg_attr(test, macro_use)]
extern crate smallvec;

use ahash::{AHashMap, AHashSet};
use arrayvec::ArrayVec;
use feather_core::util::{BlockPosition, ChunkPosition};

use feather_core::blocks::BlockId;
use feather_core::chunk::Chunk;
use feather_core::chunk_map::{chunk_relative_pos, ChunkMap};
use feather_server_types::{BlockUpdateEvent, ChunkLoadEvent, ChunkUnloadEvent, Game};
use feather_server_util::chunks_within_distance;
use parking_lot::{RwLock, RwLockWriteGuard};
use smallvec::SmallVec;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::sync::Arc;

#[fecs::event_handler]
pub fn on_block_update_notify_lighting_worker(
    event: &BlockUpdateEvent,
    #[default] handle: &LightingWorkerHandle,
) {
    let (pos, old, new) = (event.pos, event.old, event.new);
    handle
        .tx
        .send(Request::HandleBlockUpdate { pos, old, new })
        .expect("failed to notify lighting worker of block update");
}

#[fecs::event_handler]
pub fn on_chunk_load_notify_lighting_worker(
    event: &ChunkLoadEvent,
    game: &mut Game,
    handle: &LightingWorkerHandle,
) {
    let chunk_handle = game
        .chunk_map
        .chunk_handle_at(event.chunk)
        .expect("chunk load event triggered, but chunk not in chunk map");

    handle
        .tx
        .send(Request::LoadChunk {
            pos: event.chunk,
            handle: chunk_handle,
        })
        .expect("failed to notify lighting worker of chunk load");
}

#[fecs::event_handler]
pub fn on_chunk_unload_notify_lighting_worker(
    event: &ChunkUnloadEvent,
    handle: &LightingWorkerHandle,
) {
    handle
        .tx
        .send(Request::UnloadChunk { pos: event.chunk })
        .expect("failed to notify lighting worker of chunk unload");
}

/// A request sent to the lighting worker.
pub enum Request {
    /// Notifies the worker of a new loaded chunk.
    LoadChunk {
        pos: ChunkPosition,
        handle: ChunkHandle,
    },
    /// Notifies the worker that a chunk was unloaded.
    UnloadChunk { pos: ChunkPosition },
    /// Requests that the lighting worker shuts down.
    ShutDown,
    /// Requests that the lighting worker handles a block update.
    HandleBlockUpdate {
        /// The position of the block which was updated.
        pos: BlockPosition,
        /// The old value of the block.
        old: BlockId,
        /// The new value of the block.
        new: BlockId,
    },
}

/// Handle to the lighting worker.
#[derive(Clone)]
pub struct LightingWorkerHandle {
    pub tx: crossbeam::Sender<Request>,
    pub shutdown_rx: crossbeam::Receiver<()>,
}

impl Default for LightingWorkerHandle {
    fn default() -> Self {
        start_worker()
    }
}

/// Starts the lighting worker, returning a handle to it.
fn start_worker() -> LightingWorkerHandle {
    let (tx, rx) = crossbeam::bounded(512);
    let (shutdown_tx, shutdown_rx) = crossbeam::bounded(1);

    std::thread::spawn(move || run_worker(rx, shutdown_tx));

    LightingWorkerHandle { tx, shutdown_rx }
}

/// Cache storing the light sources in each chunk.
#[derive(Debug, Default)]
struct ChunkLights(AHashMap<ChunkPosition, SmallVec<[BlockPosition; 8]>>);

impl ChunkLights {
    /// Returns an iterator over light sources within the given radius
    /// of a block position.
    pub fn lights_within_radius<'a>(
        &'a self,
        pos: BlockPosition,
        radius: u8,
    ) -> impl Iterator<Item = BlockPosition> + 'a + Clone {
        let radius = f64::from(radius);
        let chunks = chunks_within_distance(pos.position(), glm::vec3(radius, radius, radius));

        chunks
            .into_iter()
            .flat_map(move |chunk| {
                self.0
                    .get(&chunk)
                    .map(|vec| vec.as_slice())
                    .unwrap_or(&[])
                    .iter()
            })
            .copied()
    }
}

/// Internal worker state.
struct Worker {
    /// Receiver for new requests.
    rx: crossbeam::Receiver<Request>,
    /// The worker's own copy of the chunk map, with `Arc`s
    /// being cloned from the server thread's "official" chunk map.
    chunk_map: ChunkMap,
    /// Caches the light sources in each chunk.
    lights: ChunkLights,
    /// Whether the worker should shut down.
    should_shut_down: bool,
}

fn run_worker(rx: crossbeam::Receiver<Request>, shutdown_tx: crossbeam::Sender<()>) {
    let mut worker = Worker {
        rx,
        chunk_map: Default::default(),
        lights: Default::default(),
        should_shut_down: false,
    };

    log::info!("Lighting worker started");
    while let Ok(request) = worker.rx.recv() {
        handle_request(&mut worker, request);

        if worker.should_shut_down {
            break;
        }
    }

    log::info!("Lighting worker shutting down");
    let _ = shutdown_tx.try_send(());
}

fn handle_request(worker: &mut Worker, request: Request) {
    match request {
        Request::ShutDown => worker.should_shut_down = true,
        Request::LoadChunk { pos, handle } => load_chunk(worker, pos, handle),
        Request::UnloadChunk { pos } => unload_chunk(worker, pos),
        Request::HandleBlockUpdate { pos, old, new } => handle_block_update(worker, pos, old, new),
    }
}

fn load_chunk(worker: &mut Worker, pos: ChunkPosition, handle: Arc<RwLock<Chunk>>) {
    worker
        .lights
        .0
        .insert(pos, lights_in_chunk(&*handle.read()).collect());
    worker.chunk_map.0.insert(pos, handle);
}

fn lights_in_chunk<'a>(chunk: &'a Chunk) -> impl Iterator<Item = BlockPosition> + 'a {
    (0..16)
        .flat_map(|x| (0..256).map(move |y| (x, y)))
        .flat_map(|(x, y)| (0..16).map(move |z| (x, y, z)))
        .filter_map(move |(x, y, z)| {
            let block = chunk.block_at(x, y, z);

            if block.light_emission() > 0 {
                Some(BlockPosition::new(x as i32, y as i32, z as i32))
            } else {
                None
            }
        })
}

fn unload_chunk(worker: &mut Worker, pos: ChunkPosition) {
    worker.lights.0.remove(&pos);
    worker.chunk_map.0.remove(&pos);
}

/// Lighter context, used to cache things during
/// a lighting iteration.
struct Context<'a> {
    /// Reference to the current cached chunk.
    /// This is used to avoid repetitive hashmap
    /// accesses in the chunk map when groups
    /// of clustered blocks are queried for.
    current_chunk: RwLockWriteGuard<'static, Chunk>,

    chunk_map: *const ChunkMap,

    _phantom: PhantomData<&'a ()>,
}

impl<'a> Context<'a> {
    fn new(chunk_map: &'a ChunkMap, start_chunk: ChunkPosition) -> Option<Self> {
        Some(Self {
            current_chunk: unsafe {
                std::mem::transmute::<RwLockWriteGuard<'a, Chunk>, RwLockWriteGuard<'static, Chunk>>(
                    chunk_map.chunk_at_mut(start_chunk)?,
                )
            },
            chunk_map: chunk_map as *const _,
            _phantom: PhantomData,
        })
    }

    fn chunk_at_mut(&mut self, pos: ChunkPosition) -> Option<&mut Chunk> {
        if pos == self.current_chunk.position() {
            Some(&mut *self.current_chunk)
        } else {
            self.current_chunk = unsafe { &*self.chunk_map }.chunk_at_mut(pos)?;
            Some(&mut *self.current_chunk)
        }
    }

    fn block_light_at(&mut self, pos: BlockPosition) -> u8 {
        match self.chunk_at_mut(pos.chunk()) {
            Some(chunk) => {
                let (x, y, z) = chunk_relative_pos(pos);
                chunk.block_light_at(x, y, z)
            }
            None => 0, // TODO: graceful handling of missing chunk information?
        }
    }

    fn set_block_light_at(&mut self, pos: BlockPosition, value: u8) {
        if let Some(chunk) = self.chunk_at_mut(pos.chunk()) {
            let (x, y, z) = chunk_relative_pos(pos);
            chunk.set_block_light_at(x, y, z, value);
        }
    }

    fn block_at(&mut self, pos: BlockPosition) -> BlockId {
        match self.chunk_at_mut(pos.chunk()) {
            Some(chunk) => {
                let (x, y, z) = chunk_relative_pos(pos);
                chunk.block_at(x, y, z)
            }
            None => BlockId::air(),
        }
    }

    fn set_block_at(&mut self, pos: BlockPosition, block: BlockId) {
        if let Some(chunk) = self.chunk_at_mut(pos.chunk()) {
            let (x, y, z) = chunk_relative_pos(pos);
            chunk.set_block_at(x, y, z, block);
        }
    }
}

const MAX_TRAVEL_DISTANCE: u8 = 15;

fn handle_block_update(worker: &mut Worker, pos: BlockPosition, old: BlockId, new: BlockId) {
    let mut ctx = match Context::new(&worker.chunk_map, pos.chunk()) {
        Some(ctx) => ctx,
        None => return, // Unloaded chunk
    };

    // Determine which algorithm to use.
    if old.light_emission() < new.light_emission() {
        ctx.set_block_light_at(pos, new.light_emission());
        emitting_creation(&mut ctx, pos);
    } else if new.light_emission() == 0 && old.light_emission() > 0 {
        ctx.set_block_light_at(pos, 0);
        emitting_removal(&mut ctx, &worker.lights, pos, old);
    } else if old.is_opaque() && !new.is_opaque() {
        opaque_non_emitting_removal(&mut ctx, pos);
    } else {
        opaque_non_emitting_creation(&mut ctx, &worker.lights, pos, new);
    }

    // Update `ChunkLights`.
    if old.light_emission() != new.light_emission() {
        if new.light_emission() == 0 {
            worker
                .lights
                .0
                .entry(pos.chunk())
                .or_default()
                .retain(|p| *p != pos);
        } else if old.light_emission() == 0 {
            worker.lights.0.entry(pos.chunk()).or_default().push(pos);
        }
    }
}

/// Algorithm #1, as described in the module-level docs.
fn emitting_creation(context: &mut Context, position: BlockPosition) {
    let emission = context.block_light_at(position);
    // Perform flood fill starting from `position`.
    // For each block, set the light value to the maximum light
    // value of any adjacent block minus 1.
    flood_fill(context, position, emission, |ctx, pos| {
        let light = light_value_for_block(ctx, pos);
        ctx.set_block_light_at(pos, light);
    });
}

/// Algorithm #2, as described in the module-level docs.
fn emitting_removal(
    context: &mut Context,
    chunk_lights: &ChunkLights,
    position: BlockPosition,
    old_block: BlockId,
) {
    // Perform flood fill and set all blocks affected by the old light to 0 light.
    flood_fill(context, position, old_block.light_emission(), |ctx, pos| {
        ctx.set_block_light_at(pos, 0);
    });

    // For all lights which could have affected the blocks we just set to 0,
    // recalculate lighting using algorithm #1.
    let nearby_lights = chunk_lights.lights_within_radius(position, MAX_TRAVEL_DISTANCE * 2);

    for light in nearby_lights {
        if light != position {
            emitting_creation(context, light);
        }
    }
}

/// Algorithm #3, as described in the module-level docs.
fn opaque_non_emitting_creation(
    context: &mut Context,
    chunk_lights: &ChunkLights,
    position: BlockPosition,
    new_block: BlockId,
) {
    // Re-calculate all lights that could have affected this block.
    // We ensure that all areas are correctly set to dark by first
    // faking that the block was never created.
    context.set_block_at(position, BlockId::air());

    let nearby_lights = chunk_lights.lights_within_radius(position, MAX_TRAVEL_DISTANCE);

    for light in nearby_lights {
        let block = context.block_at(light);
        emitting_removal(context, chunk_lights, light, block);
    }

    // Set block back to correct value.
    context.set_block_at(position, new_block);

    let nearby_lights = chunk_lights.lights_within_radius(position, MAX_TRAVEL_DISTANCE);

    // Recalculate nearby lights.
    for light in nearby_lights {
        emitting_creation(context, light);
    }
}

/// Algorithm #4, as described in the module-level docs.
fn opaque_non_emitting_removal(context: &mut Context, position: BlockPosition) {
    let value = light_value_for_block(context, position);

    context.set_block_light_at(position, value);

    // Propagate new light value for this block, as if it were a new light source.
    if value > 0 {
        emitting_creation(context, position);
    }
}

/// Returns the light value for the block at `position`,
/// equivalent to the maximum light value of an adjacent block
/// minus 1.
fn light_value_for_block(context: &mut Context, position: BlockPosition) -> u8 {
    // Find highest light value of 6 adjacent blocks.
    let adjacent = adjacent_blocks(position);

    let mut value = adjacent
        .into_iter()
        .map(|pos| context.block_light_at(pos))
        .max()
        .unwrap();

    if value > 0 {
        value -= 1;
    }

    value
}

/// Performs flood fill starting at `start` and travelling up
/// to `max_dist` blocks.
///
/// For each block iterated over, the provided closure will be invoked.
/// No block will be iterated more than once.
fn flood_fill<F>(context: &mut Context, start: BlockPosition, max_dist: u8, mut f: F)
where
    F: FnMut(&mut Context, BlockPosition),
{
    // TODO: bump allocate these data structures.
    // Don't iterate over same block more than once
    let mut touched = AHashSet::with_capacity_and_hasher(64, ahash::RandomState::new());
    touched.insert(start);

    // We use a queue-based algorithm rather than a recursive
    // one.
    let mut queue = VecDeque::with_capacity(64);

    queue.push_back(start);

    while let Some(pos) = queue.pop_front() {
        let blocks = adjacent_blocks(pos);

        for pos in blocks {
            if pos.manhattan_distance(start) > max_dist as i32 {
                // Finished
                return;
            }

            // Skip if we already went over this block
            if !touched.insert(pos) {
                continue;
            }

            let block = context.block_at(pos);
            if block.is_opaque() {
                continue; // Stop iterating
            }

            // Call closure
            f(context, pos);

            // Add block to queue
            queue.push_back(pos);
        }
    }
}

/// Returns the up to six adjacent blocks to a given block position.
fn adjacent_blocks(to: BlockPosition) -> ArrayVec<[BlockPosition; 6]> {
    let offsets = [
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ];
    offsets
        .iter()
        .map(|(x, y, z)| BlockPosition::new(to.x + *x, to.y + *y, to.z + *z))
        .filter(|pos| pos.y >= 0 && pos.y < 256)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context() {
        let mut chunk_map = ChunkMap::new();

        let pos = ChunkPosition::new(0, 0);
        chunk_map.insert(Chunk::new(pos));
        let pos2 = ChunkPosition::new(0, 1);
        chunk_map.insert(Chunk::new(pos2));

        let mut ctx = Context::new(&chunk_map, pos).unwrap();

        assert_eq!(ctx.chunk_at_mut(pos).unwrap().position(), pos);
        assert_eq!(ctx.chunk_at_mut(pos2).unwrap().position(), pos2);
        assert_eq!(ctx.chunk_at_mut(pos).unwrap().position(), pos);
    }

    #[test]
    fn test_emitting_creation() {
        let chunk_map = chunk_map();
        let mut ctx = Context::new(&chunk_map, ChunkPosition::new(0, 0)).unwrap();

        let pos = BlockPosition::new(0, 100, 0);
        ctx.set_block_at(pos, BlockId::glowstone());
        ctx.set_block_light_at(pos, BlockId::glowstone().light_emission());

        emitting_creation(&mut ctx, pos);

        assert_eq!(ctx.block_light_at(BlockPosition::new(0, 99, 0)), 14);
        assert_eq!(ctx.block_light_at(BlockPosition::new(0, 99, 1)), 13);
    }

    #[test]
    fn test_opaque_non_emitting_removal() {
        let chunk_map = chunk_map();
        let mut ctx = Context::new(&chunk_map, ChunkPosition::new(0, 0)).unwrap();

        ctx.set_block_light_at(BlockPosition::new(0, 0, 0), 10);
        ctx.set_block_light_at(BlockPosition::new(0, 2, 0), 9);
        ctx.set_block_light_at(BlockPosition::new(1, 1, 0), 8);
        ctx.set_block_light_at(BlockPosition::new(-1, 1, 0), 11);
        ctx.set_block_light_at(BlockPosition::new(0, 1, 1), 0);
        ctx.set_block_light_at(BlockPosition::new(0, 1, -1), 12);
        ctx.set_block_light_at(BlockPosition::new(0, 1, 0), 15);

        opaque_non_emitting_removal(&mut ctx, BlockPosition::new(0, 1, 0));

        assert_eq!(ctx.block_light_at(BlockPosition::new(0, 1, 0)), 11);
        assert_eq!(ctx.block_light_at(BlockPosition::new(0, 1, 1)), 10);
        assert_eq!(ctx.block_light_at(BlockPosition::new(0, 1, 2)), 9);
        assert_eq!(ctx.block_light_at(BlockPosition::new(0, 1, 3)), 8);
        assert_eq!(ctx.block_light_at(BlockPosition::new(0, 1, 4)), 7);
        // ...
    }

    #[test]
    fn test_flood_fill() {
        let chunk_map = chunk_map();
        let mut ctx = Context::new(&chunk_map, ChunkPosition::new(0, 0)).unwrap();

        let mut count = 0;

        flood_fill(&mut ctx, BlockPosition::new(100, 100, 100), 1, |_, _| {
            count += 1
        });

        assert_eq!(count, 6);
    }

    #[test]
    fn test_chunk_lights() {
        let mut chunk_lights = ChunkLights::default();
        chunk_lights.0.insert(
            ChunkPosition::new(0, 0),
            smallvec![BlockPosition::new(0, 0, 0)],
        );
        chunk_lights.0.insert(
            ChunkPosition::new(1, 0),
            smallvec![BlockPosition::new(16, 0, 0)],
        );

        assert_eq!(
            chunk_lights
                .lights_within_radius(BlockPosition::new(0, 0, 0), 16)
                .collect::<Vec<_>>()
                .as_slice(),
            &[BlockPosition::new(0, 0, 0), BlockPosition::new(16, 0, 0)]
        );
    }

    fn chunk_map() -> ChunkMap {
        let mut chunk_map = ChunkMap::new();

        for x in -1..=1 {
            for z in -1..=1 {
                let pos = ChunkPosition::new(x, z);
                chunk_map.insert(Chunk::new(pos));
            }
        }

        chunk_map
    }
}
