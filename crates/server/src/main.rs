use base::{Biome, BlockId, Chunk, ChunkPosition, World};
use common::{Game, TickLoop};
use ecs::SystemExecutor;
use feather_server::{Options, Server};
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();

    let server = Server::bind(Options::default()).await?;
    let mut game = Game::new();
    let mut systems = SystemExecutor::new();

    add_spawn_chunks(&mut game.world);

    common::register(&mut game, &mut systems);
    server.link_with_game(&mut game, &mut systems);

    let tick_loop = TickLoop::new(move || {
        systems.run(&mut game);
        false
    });

    tick_loop.run();

    Ok(())
}

fn add_spawn_chunks(world: &mut World) {
    for cx in -8..=8 {
        for cz in -8..=8 {
            let mut chunk = Chunk::new_with_default_biome(ChunkPosition::new(cx, cz), Biome::Swamp);
            for y in 0..64 {
                for z in 0..16 {
                    for x in 0..16 {
                        chunk.set_block_at(x, y, z, BlockId::grass_block());
                    }
                }
            }
            world.insert_chunk(chunk);
        }
    }
}
