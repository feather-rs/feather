//! Systems linking a `Server` and a `Game`.

use std::sync::Arc;

use base::{Biome, BlockId, Chunk, ChunkPosition, Gamemode, Position};
use common::Game;
use ecs::{SysResult, SystemExecutor};
use parking_lot::RwLock;

use crate::Server;

/// Registers systems for a `Server` with a `Game`.
pub fn register(server: Server, game: &mut Game, systems: &mut SystemExecutor<Game>) {
    game.insert_resource(server);
    systems.group::<Server>().add_system(poll_new_players);
}

/// Polls for new clients and sends them the necessary packets
/// to join the game.
fn poll_new_players(game: &mut Game, server: &mut Server) -> SysResult {
    for client in server.accept_new_players() {
        let client = server.clients.get(client).unwrap();
        client.send_join_game(0, Gamemode::Creative);
        client.send_brand();
        client.uupdate_own_chunk(ChunkPosition::new(0, 0));

        for x in -4..=4 {
            for z in -4..=4 {
                let mut chunk = Chunk::new_with_default_biome(ChunkPosition::new(x, z), Biome::Badlands);
                for by in 0..64 {
                    for bz in 0..16 {
                        for bx in 0..16 {
                            chunk.set_block_at(bx, by, bz, BlockId::stone());
                        }
                    }
                }
                client.send_chunk(&Arc::new(RwLock::new(chunk)));
            }
        }

        client.update_own_position(Position::default());
    }

    Ok(())
}
