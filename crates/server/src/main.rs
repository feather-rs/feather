use common::{world_source::flat::FlatWorldSource, Game, TickLoop, World};
use ecs::SystemExecutor;
use feather_server::{Options, Server};
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Trace)
        .init()
        .unwrap();

    let server = Server::bind(Options::default()).await?;
    let mut game = Game::new();
    game.world = World::with_source(FlatWorldSource::new());
    let mut systems = SystemExecutor::new();

    common::register(&mut game, &mut systems);
    server.link_with_game(&mut game, &mut systems);
    print_systems(&mut systems);

    let tick_loop = TickLoop::new(move || {
        systems.run(&mut game);
        log::info!(
            "#Loaded chunks: {}",
            game.world.chunk_map().iter_chunks().into_iter().count()
        );
        false
    });

    tick_loop.run();

    Ok(())
}

fn print_systems(systems: &mut SystemExecutor<Game>) {
    let systems: Vec<&str> = systems.system_names().collect();
    log::debug!("---SYSTEMS---\n{:#?}\n", systems);
}
