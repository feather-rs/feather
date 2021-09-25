use anyhow::{bail, Context};
use argh::FromArgs;
use cargo_metadata::Metadata;
use heck::CamelCase;
use quill_plugin_format::{PluginFile, PluginMetadata, PluginTarget, Triple};
use std::{
    fs,
    path::PathBuf,
    process::{Command, Stdio},
};

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
    Build(Build),
}

#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "build")]
/// Build a Quill plugin.
struct Build {
    #[argh(switch)]
    /// whether to build in release mode
    release: bool,
    #[argh(switch)]
    /// whether to compile to a native shared library
    /// instead of a WebAssembly module
    native: bool,
    #[argh(option, default = "6")]
    /// the compression level to compress the plugin
    /// binary. 0 is worst and 9 is best.
    compression_level: u32,
}

impl Build {
    pub fn module_extension(&self) -> &'static str {
        if !self.native {
            "wasm"
        } else if cfg!(windows) {
            "dll"
        } else if cfg!(target_vendor = "apple") {
            "dylib"
        } else {
            // assume Linux / other Unix
            "so"
        }
    }

    pub fn target_dir(&self, cargo_meta: &Metadata) -> PathBuf {
        let mut target_dir = cargo_meta.target_directory.clone();
        if !self.native {
            target_dir.push(WASM_TARGET);
        }

        if self.release {
            target_dir.push("release");
        } else {
            target_dir.push("debug");
        }

        target_dir
    }

    pub fn module_path(&self, cargo_meta: &Metadata, plugin_meta: &PluginMetadata) -> PathBuf {
        let target_dir = self.target_dir(cargo_meta);
        let module_filename = plugin_meta.identifier.replace("-", "_");

        let module_extension = self.module_extension();
        let lib_prefix = if self.native && cfg!(unix) { "lib" } else { "" };

        target_dir.join(format!(
            "{}{}.{}",
            lib_prefix, module_filename, module_extension
        ))
    }
}

fn main() -> anyhow::Result<()> {
    let args: CargoQuill = argh::from_env();
    match args.subcommand {
        Subcommand::Build(args) => build(args),
    }
}

fn build(args: Build) -> anyhow::Result<()> {
    let cargo_meta = get_cargo_metadata()?;
    validate_cargo_metadata(&cargo_meta)?;

    let mut command = cargo_build_command(&args);
    let status = command.spawn()?.wait()?;
    if !status.success() {
        bail!("build failed");
    }

    let meta = find_metadata(&cargo_meta, &args)?;
    let module_path = args.module_path(&cargo_meta, &meta);
    let module = fs::read(&module_path)
        .with_context(|| format!("failed to read {}", module_path.display()))?;

    let file = PluginFile::new(module, meta.clone());
    let target_path = module_path
        .parent()
        .unwrap()
        .join(format!("{}.plugin", meta.identifier));
    fs::write(&target_path, file.encode(args.compression_level))?;

    println!("Wrote plugin file to {}", target_path.display());
    Ok(())
}

fn cargo_build_command(args: &Build) -> Command {
    let mut cmd = Command::new("cargo");
    cmd.arg("rustc");
    if args.release {
        cmd.arg("--release");
    }

    if !args.native {
        cmd.args(&["--target", WASM_TARGET]);
        cmd.args(&["--", "-C", WASM_TARGET_FEATURES]);
    }

    cmd.stdout(Stdio::piped());

    cmd
}

fn get_cargo_metadata() -> anyhow::Result<Metadata> {
    let cmd = cargo_metadata::MetadataCommand::new();
    let cargo_meta = cmd.exec()?;
    Ok(cargo_meta)
}

fn validate_cargo_metadata(cargo_meta: &Metadata) -> anyhow::Result<()> {
    let package = cargo_meta.root_package().context("missing root package")?;
    if !package
        .targets
        .iter()
        .any(|t| t.crate_types.contains(&"cdylib".to_owned()))
    {
        bail!("crate-type = [\"cdylib\"] must be set in the plugin Cargo.toml");
    }

    Ok(())
}

fn find_metadata(cargo_meta: &Metadata, args: &Build) -> anyhow::Result<PluginMetadata> {
    let package = cargo_meta.root_package().context("missing root package")?;

    let quill_dependency = package
        .dependencies
        .iter()
        .find(|d| d.name == "quill")
        .context("plugin does not depend on the `quill` crate")?;

    let target = if args.native {
        PluginTarget::Native {
            target_triple: Triple::host(),
        }
    } else {
        PluginTarget::Wasm
    };

    let plugin_meta = PluginMetadata {
        name: package.name.to_camel_case(),
        identifier: package.name.clone(),
        version: package.version.to_string(),
        api_version: quill_dependency.req.to_string(),
        description: package.description.clone(),
        authors: package.authors.clone(),
        target,
    };

    Ok(plugin_meta)
}
