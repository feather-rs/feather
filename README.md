# Feather
[![build](https://github.com/feather-rs/feather/workflows/build/badge.svg)](https://github.com/feather-rs/feather/actions)
[![Discord](https://img.shields.io/discord/619316022800809995)](https://discordapp.com/invite/4eYmK69)

An experimental Minecraft server implementation written in Rust.

### Features
Many basic features are already implemented:
- [x] Highly scalable architecture
- [x] Anvil world loading and saving
- [x] Physics
- [x] Basic world generation
- [x] Chunk streaming
- [x] Day/night cycle
- [x] Arrow shooting
- [x] Falling blocks
- [x] Block placement and breaking
- [x] Item dropping and collection
- [x] Chat
- [x] Inventory handling
- [x] Movement broadcasting

### Running
We offer precompiled binaries for Windows and Linux at [GitHub Releases](https://github.com/caelunshun/feather/releases).

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

The server executable will be located in `target/release`.
