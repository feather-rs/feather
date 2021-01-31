use common::{Game, TickLoop};
use feather_server::{Options, Server};
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();

    let server = Server::bind(Options::default()).await?;
    let game = Game::new();

    let tick_loop = TickLoop::new(move || false);

    tick_loop.run();

    Ok(())
}
