//! Implements block change broadcasting.
//!
//! # Bulk updates
//! The protocol provides three methods to change blocks
//! on the client:
//! * The `BlockChange` packet to update a single block.
//! * The `MultiBlockChange` packet to update multiple blocks
//! within a single chunk section.
//! * The `ChunkData` packet to overwrite entire chunk sections
//! at once.
//!
//! Feather is optimized for bulk block updates to cater to plugins
//! like WorldEdit. This module chooses the optimal packet from
//! the above three options to achieve ideal performance.

use ahash::AHashMap;
use base::{chunk::SECTION_VOLUME, position, ChunkPosition, CHUNK_WIDTH};
use common::{events::BlockChangeEvent, Game};
use ecs::{SysResult, SystemExecutor};

use crate::Server;

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems
        .group::<Server>()
        .add_system(broadcast_block_changes);
}

fn broadcast_block_changes(game: &mut Game, server: &mut Server) -> SysResult {
    for (_, event) in game.ecs.query::<&BlockChangeEvent>().iter() {
        broadcast_block_change(event, game, server);
    }
    Ok(())
}

/// Threshold at which to switch from block change to chunk
// overwrite packets.
const CHUNK_OVERWRITE_THRESHOLD: usize = SECTION_VOLUME / 2;

fn broadcast_block_change(event: &BlockChangeEvent, game: &Game, server: &mut Server) {
    if event.count() >= CHUNK_OVERWRITE_THRESHOLD {
        broadcast_block_change_chunk_overwrite(event, game, server);
    } else {
        broadcast_block_change_simple(event, game, server);
    }
}

fn broadcast_block_change_chunk_overwrite(
    event: &BlockChangeEvent,
    game: &Game,
    server: &mut Server,
) {
    let mut sections: AHashMap<ChunkPosition, Vec<usize>> = AHashMap::new();
    for (chunk, section, _) in event.iter_affected_chunk_sections() {
        sections.entry(chunk).or_default().push(section + 1); // + 1 to account for the void air chunk
    }

    for (chunk_pos, sections) in sections {
        let chunk = game.world.chunk_map().chunk_handle_at(chunk_pos);
        if let Some(chunk) = chunk {
            let position = position!(
                (chunk_pos.x * CHUNK_WIDTH as i32) as f64,
                0.0,
                (chunk_pos.z * CHUNK_WIDTH as i32) as f64,
            );
            server.broadcast_nearby_with(position, |client| {
                client.overwrite_chunk_sections(&chunk, sections.clone());
            })
        }
    }
}

fn broadcast_block_change_simple(event: &BlockChangeEvent, game: &Game, server: &mut Server) {
    for pos in event.iter_changed_blocks() {
        let new_block = game.block(pos);
        if let Some(new_block) = new_block {
            server.broadcast_nearby_with(pos.position(), |client| {
                client.send_block_change(pos, new_block)
            });
        }
    }
}
