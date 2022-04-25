use feather_server::ServerBuilder;

fn main() -> anyhow::Result<()> {
    ServerBuilder::new()?.register_default_plugins().run()
}
