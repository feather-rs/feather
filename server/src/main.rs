use tokio::runtime;

fn main() {
    // Start Tokio runtime, then call lib::main().
    let mut runtime = runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .expect("failed to start Tokio runtime");

    let handle = runtime.handle().clone();

    runtime.block_on(async move {
        feather_server::main(handle).await;
    });
}
