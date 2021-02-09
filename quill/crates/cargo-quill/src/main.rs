use anyhow::{bail, Context};
use argh::FromArgs;
use heck::CamelCase;
use quill_plugin_format::{PluginFile, PluginMetadata};
use std::{
    fs,
    path::PathBuf,
    process::{Command, Stdio},
};

const TARGET_FEATURES: &str = "target-feature=+bulk-memory,+mutable-globals,+simd128";
const TARGET: &str = "wasm32-unknown-unknown";

const COMPRESSION_LEVEL: u32 = 6;

#[derive(Debug, FromArgs)]
/// Cargo subcommand to build and test Quill/Feather plugins.
struct CargoQuill {
    #[argh(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, FromArgs)]
#[argh(subcommand)]
enum Subcommand {
    Build(Build),
}

#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "build")]
/// Build a Quill plugin.
struct Build {
    #[argh(switch)]
    /// whether to build in release mode
    release: bool,
}

fn main() -> anyhow::Result<()> {
    let args: CargoQuill = argh::from_env();
    match args.subcommand {
        Subcommand::Build(args) => build(args),
    }
}

fn build(args: Build) -> anyhow::Result<()> {
    let mut command = cargo_build_command(&args);
    let status = command.spawn()?.wait()?;
    if !status.success() {
        bail!("build failed");
    }

    let (meta, mut target_dir) = find_metadata()?;
    target_dir.push(TARGET);
    if args.release {
        target_dir.push("release");
    } else {
        target_dir.push("debug");
    }
    let wasm_bytecode = fs::read(target_dir.join(format!("{}.wasm", meta.identifier)))
        .context("failed to read compiled WASM binary")?;

    let target_path = target_dir.join(format!("{}.plugin", meta.identifier));
    let file = PluginFile::new(wasm_bytecode, meta);
    fs::write(&target_path, file.encode(COMPRESSION_LEVEL))?;

    println!("Wrote plugin file to {}", target_path.display());
    Ok(())
}

fn cargo_build_command(args: &Build) -> Command {
    let mut cmd = Command::new("cargo");
    cmd.arg("rustc");
    if args.release {
        cmd.arg("--release");
    }

    cmd.args(&["--target", TARGET]);
    cmd.args(&["--", "-C", TARGET_FEATURES]);

    cmd.stdout(Stdio::piped());

    cmd
}

fn find_metadata() -> anyhow::Result<(PluginMetadata, PathBuf)> {
    let cmd = cargo_metadata::MetadataCommand::new();
    let cargo_meta = cmd.exec()?;
    let package = cargo_meta.root_package().context("missing root package")?;

    let quill_dependency = package
        .dependencies
        .iter()
        .find(|d| d.name == "quill")
        .context("plugin does not depend on the `quill` crate")?;

    let plugin_meta = PluginMetadata {
        name: package.name.to_camel_case(),
        identifier: package.name.clone(),
        version: package.version.to_string(),
        api_version: quill_dependency.req.to_string(),
        description: package.description.clone(),
        authors: package.authors.clone(),
    };

    Ok((plugin_meta, cargo_meta.target_directory))
}
