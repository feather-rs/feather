# Feather
[![build](https://dev.azure.com/caelunshun/feather/_apis/build/status/caelunshun.feather?branchName=develop)](https://dev.azure.com/caelunshun/feather/_build/latest?definitionId=1&branchName=develop)
[![coverage](https://codecov.io/gh/caelunshun/feather/branch/develop/graph/badge.svg)](https://codecov.io/gh/caelunshun/feather)

An experimental Minecraft server implementation written in Rust.

### Current Features
Many basic features are already implemented:
- [x] Highly scalable architecture—thanks to [`specs`](https://github.com/slide-rs/specs) and [`rayon`](https://github.com/rayon-rs/rayon), Feather is almost entirely multithreaded
- [x] Anvil world loading
- [x] Chunk streaming
- [x] Physics
- [x] Block placement and breaking
- [x] Item dropping and collection
- [x] Chat
- [x] Inventory handling
- [x] Movement broadcasting

Development is currently quite active, and features should be added at a fast pace over the next few months.

### Running
We offer precompiled binaries for Windows and Linux at [GitHub Releases](https://github.com/caelunshun/feather/releases).

To run Feather:
* Extract the downloaded archive.
* Move a 1.13.2 Minecraft world save to the directory you extracted the archive to. Name the world save "world."
* Run the binary.
  * On Linux and macOS: `./feather_server` in the server directory
  * On Windows: double-click `feather_server.exe`
  
The server will create a configuration file (`feather.toml`) which you can modify.

Feather currently only supports 1.13.2 clients and world saves. In the future, additional versions will be supported.

### Compiling
If you are on another platform, compile the server yourself to try it out:
```bash
git clone https://github.com/caelunshun/feather
cd feather
cargo build --release
```

The server executable will be located in `target/release`.