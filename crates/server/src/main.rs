use common::{
    world_source::{flat::FlatWorldSource, region::RegionWorldSource, WorldSource},
    Game, TickLoop, World,
};
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

    let world_source = RegionWorldSource::new("world").with_fallback(FlatWorldSource::new());
    game.world = World::with_source(world_source);

    let mut systems = SystemExecutor::new();
    common::register(&mut game, &mut systems);
    server.link_with_game(&mut game, &mut systems);
    print_systems(&mut systems);

    let tick_loop = TickLoop::new(move || {
        systems.run(&mut game);
        false
    });

    tick_loop.run();

    Ok(())
}

fn print_systems(systems: &mut SystemExecutor<Game>) {
    let systems: Vec<&str> = systems.system_names().collect();
    log::debug!("---SYSTEMS---\n{:#?}\n", systems);
}
