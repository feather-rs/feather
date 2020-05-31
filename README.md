# Feather
[![build](https://github.com/feather-rs/feather/workflows/build/badge.svg)](https://github.com/feather-rs/feather/actions)
[![Discord](https://img.shields.io/discord/619316022800809995)](https://discordapp.com/invite/4eYmK69)

A Minecraft server implementation written in Rust.

### Features

Note that Feather is still early in development. Don't expect anything not listed here to work.

- [x] Highly scalable architecture
- [x] Anvil world loading and saving
- [x] Physics
- [x] Basic world generation
- [x] Chunk streaming
- [x] Day/night cycle
- [x] Weather
- [x] Block lighting
- [x] Arrow shooting
- [x] Falling blocks
- [x] Block placement and breaking
- [x] Item dropping and collection
- [x] Chat
- [x] Inventory handling
- [x] Movement broadcasting

On the current `develop` branch, some more features are pending release:
- [x] Commands (/tell, /tp, /gamemode)
- [x] Survival mode
- [x] Survival mode block breaking and drops
- [x] Health + fall damage
- [x] (soon) block entities, including chests

### Running
We offer precompiled binaries for Windows, Linux, and macOS at [GitHub Releases](https://github.com/feather-rs/feather/releases).

To run Feather:
* Extract the downloaded archive.
* Run the binary.
  * On Linux and macOS: `./feather-server` in the server directory
  * On Windows: double-click `feather-server.exe`
  
The server will create a configuration file (`feather.toml`) which you can modify.

Feather will generate a world by default. If you want to load a vanilla world,
copy the world save to the server directory under the name "world" (by default).

Warning: Feather world persistence is fairly new and will likely cause problems
when attempting to open Feather worlds in vanilla. Do not let Feather touch worlds
you care about unless they have been backed up.

Feather currently only supports 1.13.2 clients and world saves. In the future, additional versions will be supported.

### Compiling
If you are on another platform, compile the server yourself to try it out:
```bash
git clone https://github.com/feather-rs/feather
cd feather
cargo build --release
```

Compiling from source requires Java JDK 8.

The server executable will be located in `target/release`.

### FAQ

* Is Feather production ready?

Not yet. There are numerous bugs and missing features which have yet to be resolved,
and the codebase has not been tested enough to consider the server production ready.

* How can I contribute?

Check out our [issue tracker](https://github.com/feather-rs/feather/issues) to find an issue which appeals to you. Feel free
to join [our Discord](https://discordapp.com/invite/4eYmK69) and ask questions whenever you need. Thanks for your interest in contributing!

* Are there other ways I can help?

Yes! We're always looking for people to test out the server and find bugs. If you find anything that doesn't
seem right to you, please submit an issue on the issue tracker.
