use anyhow::bail;
use argh::FromArgs;
use std::{
    fs,
    process::{Command, Stdio},
};

use std::io::Write;

const QUILL_DEPENDENCY: &str =
    "quill = {git = \"https://github.com/feather-rs/feather.git\", branch = \"main\"}\n";
const QUILL_CDYNLIB: &str = "[lib]\ncrate-type = [\"cdylib\"]\n";

#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "new")]
/// Build a Quill plugin.
pub struct New {
    #[argh(positional)]
    name: String,
}

pub fn new_command(args: New) -> anyhow::Result<()> {
    create_new_crate(args)
}

fn create_new_crate(args: New) -> anyhow::Result<()> {
    let cwd = match std::env::current_dir() {
        Ok(x) => x,
        Err(_) => bail!("Unable to get the current directory using std::env."),
    };

    if !run_cargo_new_command(args.name.as_str())? {
        // Then cargo new failed
        bail!("Cargo new failed for the given name {}", args.name);
    }

    // The new library was created time to modify the cargo.toml
    // file to contain quill as a dependency, and make it a cdylib

    let mut cargo_toml = cwd.clone();
    cargo_toml.push(args.name.as_str());
    cargo_toml.push("Cargo.toml");

    assert!(cwd.exists());

    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(cargo_toml)
        .unwrap();

    write!(file, "{}", QUILL_DEPENDENCY)?;
    write!(file, "{}", QUILL_CDYNLIB)?;

    Ok(())
}

fn run_cargo_new_command(name: &str) -> anyhow::Result<bool> {
    let mut cmd = Command::new("cargo");
    cmd.arg("new");
    cmd.arg("--lib");
    cmd.arg(name);

    cmd.stdout(Stdio::piped());

    let res = cmd.spawn()?.wait()?;
    Ok(res.success())
}
