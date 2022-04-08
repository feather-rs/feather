use feather_server::ServerBuilder;

fn main() -> anyhow::Result<()> {
    ServerBuilder::new()?.run()
}
