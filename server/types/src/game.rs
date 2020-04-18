use feather_chunk_map::ChunkMap;

/// The `Game` resource, which acts as a central bus to bind together
/// the feather-server-* crates. Resources which are accessed frequently,
/// such as the chunk map, are stored in here.
pub struct Game {
    pub chunk_map: ChunkMap,
    /// Number of ticks since the program started. Can be used
    /// to make a system which only runs at a fixed interval.
    pub tick_count: u64,
}
