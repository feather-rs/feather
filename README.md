# Feather
[![build](https://github.com/feather-rs/feather/workflows/build/badge.svg)](https://github.com/feather-rs/feather/actions)
[![Discord](https://img.shields.io/discord/619316022800809995?logo=discord)](https://discordapp.com/invite/4eYmK69)

A Minecraft server implementation written in Rust.

### Status

The project is in an **early stage**. Many, many features are unimplemented. We welcome help from anyone willing to contribute!

### Supported Minecraft versions

Feather supports 1.16.5 clients and world saves. We do not currently have plans to support multiple versions at once, but
we may consider this in the future.

### Goals

The Feather project aims to provide a Minecraft server that is _fast_, _modular_, and paired with an ergonomic plugin API.

Our mid-term goal is to make Feather usable on hub and minigame servers. The limited set of gameplay features available in Feather
is not a problem for such servers that require a small subset of vanilla functionality. On the other hand, Feather's modularity
and performance lends itself to these types of servers. Therefore, our current focus is
on building a rich plugin API to enable these use cases.

In the long term, Feather could be used on larger, more survival-like servers, where its performance should allow many players to simultaneously play on the same world requiring very few resources.

### Ecosystem

The Feather ecosystem consists of several repositories:
* [`libcraft`](https://github.com/feather-rs/feather/tree/main/libcraft), a set of Rust crates providing Minecraft functionality.
* [`quill`](https://github.com/feather-rs/feather/tree/main/quill), our work-in-progress plugin API. Quill plugins are written in Rust and compiled to WebAssembly. Feather runs them in a sandboxed WebAssembly VM.
* `feather`, the server software built on top of `libcraft` and `quill`.

### Performance

Comparisons to vanilla performance _will_ be extremely misleading, because Feather implements so few features. But if you really want them:

* Feather can handle 1,000,000 entities spawned by a plugin before it starts to max out the CPU. The vanilla server will croak long before then.
* Feather can handle 500 concurrent player connections with each player walking in a random direction.

These results _will_ change after more features are implemented in Feather, so take them with a grain of salt.

Memory usage in Feather is proportional to the number of loaded chunks, not player counts. In the 500 player test, the server uses ~40 MiB of RAM
until the players start to spread out. In the 1,000,000 entities test, it uses 400 MiB of RAM without any chunks loaded.

### Running
We offer precompiled binaries for Windows, Linux, and macOS at [GitHub Actions](https://github.com/feather-rs/feather/actions/workflows/main.yml).
NB: Do **NOT** use github releases, they are majorly outdated

To run Feather:
* Extract the downloaded archive.
* Run the binary.
  * On Linux and macOS: `./feather-server` in the server directory
  * On Windows: double-click `feather-server.exe`
  
The server will create a configuration file (`config.toml`) which you can modify.

Feather will generate a world by default. If you want to load a vanilla world,
copy the world save to the server directory under the name "world" (by default).

Warning: Feather world persistence is fairly new and will likely cause problems
when attempting to open Feather worlds in vanilla. Do not let Feather touch worlds
you care about unless they have been backed up.

### Compiling
If you are on another platform, compile the server yourself to try it out:
```bash
git clone https://github.com/feather-rs/feather
cd feather
cargo build --release
```

Compiling from source requires the latest stable version of Rust. Older Rust versions may be able
to compile Feather, but they are not guaranteed to keep working.

The server executable will be located in `target/release`.

### Architecture

For contributors, we have a work-in-progress explanation of Feather's architecture [here](docs/architecture.md).

### FAQ

* Is Feather production ready?

Not yet. There are numerous bugs and missing features which have yet to be resolved,
and the codebase has not been tested enough to consider the server production ready.

* How can I contribute?

Check out our [issue tracker](https://github.com/feather-rs/feather/issues) to find out what needs to be worked on. Feel free
to join [our Discord](https://discordapp.com/invite/4eYmK69) and ask questions whenever you need. Thanks for your interest in contributing!

* Are there other ways I can help?

Yes! We're always looking for people to test out the server and find bugs. If you find anything that doesn't
seem right to you, please submit an issue on the issue tracker.
