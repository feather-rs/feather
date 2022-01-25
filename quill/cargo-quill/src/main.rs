use argh::FromArgs;

mod build;
mod new;
mod testing;

const WASM_TARGET_FEATURES: &str = "target-feature=+bulk-memory,+mutable-globals,+simd128";
const WASM_TARGET: &str = "wasm32-wasi";

#[derive(Debug, FromArgs)]
/// Cargo subcommand to build and test Quill/Feather plugins.
struct CargoQuill {
    #[argh(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, FromArgs)]
#[argh(subcommand)]
enum Subcommand {
    Build(build::Build),
    New(new::New),
    Test(testing::Testing),
}

fn main() -> anyhow::Result<()> {
    let args: CargoQuill = argh::from_env();
    match args.subcommand {
        Subcommand::Build(args) => build::build(args),
        Subcommand::New(args) => new::new_command(args),
        Subcommand::Test(args) => testing::test_command(args),
    }
}
