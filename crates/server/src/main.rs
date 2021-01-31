use common::{Game, TickLoop};
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
    let mut systems = SystemExecutor::new();

    server.link_with_game(&mut game, &mut systems);

    let tick_loop = TickLoop::new(move || {
        systems.run(&mut game);
        false
    });

    tick_loop.run();

    Ok(())
}
