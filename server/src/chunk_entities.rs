use dashmap::DashMap;
use feather_core::ChunkPosition;
use legion::entity::Entity;
use parking_lot::Mutex;
use thread_local::ThreadLocal;

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
/// `dashmap` for concurrent access.
///
/// Do note that the information in this structure is not necessarily up to date,
/// although a best effort is made to update the data.
#[derive(Resource)]
pub struct ChunkEntities(DashMap<ChunkPosition, Vec<Entity>>);

impl ChunkEntities {
    pub fn new() -> Self {
        Self(DashMap::default())
    }

    /// Returns a slice of entities in the given chunk.
    pub fn entities_in_chunk(&self, chunk: ChunkPosition) -> &[Entity] {
        todo!("implement chunk entities properly");
        if let Some(vec) = self.0.get(&chunk) {
            vec.as_slice()
        } else {
            &EMPTY_VEC
        }
    }
}

impl Default for ChunkEntities {
    fn default() -> Self {
        Self::new()
    }
}
