# Getting started

## Install Rust

First you should install Rust, you can follow the guide below or visit [rustup.rs](https://rustup.rs/) directly.

### To install Rust, if you are running Unix,
run the following in your terminal, then follow the onscreen instructions.
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### If you are running Windows 64-bit,
download and run \
[rustup-init.exe](https://win.rustup.rs/x86_64) \
then follow the onscreen instructions.

### If you are running Windows 32-bit,
download and run \
[rustup-init.exe](https://win.rustup.rs/i686) \
then follow the onscreen instructions. 

## Install Quill
Second you are should install the Quill utility.

To install Quill utility run the following in your terminal.
```sh
cargo install cargo-quill
```

## Creating a new plugin
To create a new plugin in an exisiting directory
```sh
cargo quill init [path]
```

This command will create a new Cargo cdylib in the current directory.
Give a path as an argument to create in the given directory.

It creates will populate the directory with the following files.
```
src/
└── lib.rs
.gitignore
cargo.toml
```

## Recomendations
We also recomend that you install the following
### Rust analyzer
rust-analyzer is an experimental modular compiler frontend for the Rust language. It is a part of a larger rls-2.0 effort to create excellent IDE support for Rust.

Usally your IDE can install rust analyzer, if not follow this [guide](https://rust-analyzer.github.io/manual.html#installation).
### Rustfmt
A tool for formatting Rust code according to style guidelines.

To install rustfmt run the following in your terminal.
```sh
rustup component add rustfmt
```
### Clippy
A collection of lints to catch common mistakes and improve your Rust code.

To install clippy run the following in your terminal.
```sh
rustup component add clippy
```

## Plugin identifier
The `name` in the manifest will be used as the plugin identifier when publishing the plugin. The `name` must adhere to the following regex `[a-z][a-z0-9_-]*` to be published on `feathermc.org/plugins`