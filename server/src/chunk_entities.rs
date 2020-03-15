use crate::entity::{EntityCreateEvent, EntityDeleteEvent, EntityMoveEvent, PreviousPosition};
use crate::state::State;
use feather_core::{ChunkPosition, Position};
use hashbrown::HashMap;
use legion::entity::Entity;
use legion::query::Read;
use parking_lot::{MappedRwLockReadGuard, RwLock, RwLockReadGuard};
use rayon::prelude::*;
use tonks::{PreparedWorld, Query};

static EMPTY_VEC: Vec<Entity> = Vec::new();

/// Stores which entities belong to every given chunk.
///
/// This data structure can be used to accelerate certain
/// operations, such as querying for entities
/// within some distance of a position. In addition,
/// it can be used to send all entities in a chunk
/// to a player.
///
/// This structure is internally stored in `State`, using
/// a `RwLock` for concurrent access. (TODO: remove lock.)
///
/// Do note that the information in this structure is not necessarily up to date,
/// although a best effort is made to update the data.
#[derive(Resource)]
pub struct ChunkEntities(RwLock<HashMap<ChunkPosition, Vec<Entity>>>);

impl ChunkEntities {
    pub fn new() -> Self {
        Self(RwLock::new(HashMap::new()))
    }

    /// Returns a slice of entities in the given chunk.
    pub fn entities_in_chunk(&self, chunk: ChunkPosition) -> MappedRwLockReadGuard<[Entity]> {
        let map = self.0.read();

        RwLockReadGuard::map(map, move |map| {
            if let Some(vec) = map.get(&chunk) {
                vec.as_slice()
            } else {
                &EMPTY_VEC
            }
        })
    }
}

impl Default for ChunkEntities {
    fn default() -> Self {
        Self::new()
    }
}

/// System to update ChunkEntities when entities move into new chunks.
#[event_handler]
fn chunk_entities_handle_movement(
    events: &[EntityMoveEvent],
    state: &State,
    _query: &mut Query<(Read<Position>, Read<PreviousPosition>)>,
    world: &mut PreparedWorld,
) {
    events.par_iter().for_each(|event| {
        let old_pos = world
            .get_component::<PreviousPosition>(event.entity)
            .unwrap()
            .0;
        let new_pos = *world.get_component::<Position>(event.entity).unwrap();

        let old_chunk = old_pos.chunk();
        let new_chunk = new_pos.chunk();

        if old_chunk != new_chunk {
            // Update chunk entities
            let mut map = state.chunk_entities.0.write();
            map.entry(new_chunk)
                .or_insert_with(|| vec![])
                .push(event.entity);
            map.entry(old_chunk).and_modify(|vec| {
                vec.remove_item(&event.entity);
            });
        }
    })
}

#[event_handler]
fn chunk_entities_insert(
    event: &EntityCreateEvent,
    state: &State,
    _query: &mut Query<Read<Position>>,
    world: &mut PreparedWorld,
) {
    if let Some(position) = world.get_component::<Position>(event.entity) {
        let chunk = position.chunk();
        let mut map = state.chunk_entities.0.write();

        map.entry(chunk)
            .or_insert_with(|| vec![])
            .push(event.entity);
    }
}

#[event_handler]
fn chunk_entities_remove(event: &EntityDeleteEvent, state: &State) {
    if let Some(position) = event.position {
        let chunk = position.chunk();
        let mut map = state.chunk_entities.0.write();

        map.entry(chunk)
            .or_insert_with(|| vec![])
            .remove_item(&event.entity);
    }
}
