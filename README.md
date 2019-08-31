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

Obviously, key features are missing. However, development is currently
quite active, and features should be added at a fast pace over the next few months.

### Running
If you want to try out the software, you currently have to compile Feather yourself.

Run ```cargo build --release``` in the repository's root directory to compile the code. To start the server,
you will have to do the following:
- Move the binary ```target/release/feather_server``` to your desired server directory.
- Copy a **1.13.2** Minecraft world save to the server directory, under the directory name "world."

Then, simply run the binary: ```./feather_server```.

The server will create a configuration file (`feather.toml`) which you can modify.

The server currently only supports 1.13.2 clients and world saves. In the future, additional versions will be supported.
