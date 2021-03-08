# Getting started

## Install `rustup`

First you should install `rustup`, you can follow the guide below or visit [rustup.rs](https://rustup.rs/) directly.

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

## Install `cargo-quill`
Second you are should install the `cargo-quill` CLI utility.

To install `cargo-quill` run the following in your terminal.
```sh
cargo install cargo-quill
```

## Creating a new plugin
To create a new plugin in an exisiting directory
`cargo quill init`

This command will create a new Cargo cdylib in the current directory.
Give a path as an argument to create in the given directory.

It creates will populate the directory with the following files.
```
src/
└── lib.rs
.gitignore
cargo.toml
```

## Plugin identifier
The `name` in the manifest will be used as the plugin identifier when publishing the plugin. The `name` must adhere to the following regex `[a-z][a-z0-9_-]*` to be published on `feathermc.org/plugins`