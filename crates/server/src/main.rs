use feather_server::{Options, Server};
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();
    Server::bind(Options::default()).await?;

    std::thread::sleep(std::time::Duration::from_secs(1000));
    Ok(())
}
