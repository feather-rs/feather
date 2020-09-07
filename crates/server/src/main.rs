use simple_logger::SimpleLogger;
use tokio::runtime;

mod config;
mod network;

fn main() -> anyhow::Result<()> {
    SimpleLogger::new().init().unwrap();
    let runtime = runtime::Builder::new()
        .threaded_scheduler()
        .enable_io()
        .build()?;

    runtime.enter(|| {
        let listener =
            network::Listener::new("127.0.0.1:25565".parse().unwrap(), runtime.handle())?;
        runtime.handle().block_on(async move {
            listener.run().await;
        });
        Result::<(), anyhow::Error>::Ok(())
    })?;

    Ok(())
}
