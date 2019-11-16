use ahash::ABuildHasher;
use evmap::shallow_copy::CopyValue;
use evmap::{ReadHandle, ReadHandleFactory, WriteHandle};
use feather_core::ChunkPosition;
use legion::entity::Entity;
use thread_local::ThreadLocal;

/// Stores which entities belong to every given chunk.
///
/// This data structure can be used to accelerate certain
/// operations, such as querying for entities
/// within some distance of a position. In addition,
/// it can be used to send all entities in a chunk
/// to a player.
///
/// This structure is internally stored in `State`, using
/// `evmap` for concurrent map access.
///
/// Do note that the information in this structure is not necessarily up to date,
/// although a best effort is made to update the data.
pub struct ChunkEntities {
    writer: WriteHandle<ChunkPosition, CopyValue<Entity>, (), ABuildHasher>,
    factory: ReadHandleFactory<ChunkPosition, CopyValue<Entity>, (), ABuildHasher>,
    readers: ThreadLocal<ReadHandle<ChunkPosition, CopyValue<Entity>, (), ABuildHasher>>,
}

impl ChunkEntities {
    pub fn new() -> Self {
        let (reader, writer) = evmap::with_hasher((), ABuildHasher);
        Self {
            writer,
            factory: reader.factory(),
            readers: ThreadLocal::new(),
        }
    }
    /// Returns a slice of entities in the given chunk.
    pub fn entities_in_chunk(&self, chunk: ChunkPosition) -> &[CopyValue<Entity>] {
        self.readers
            .get_or(|| self.factory.handle())
            .get_and(&chunk, |slice| slice)
            .unwrap_or(&[])
    }
}
