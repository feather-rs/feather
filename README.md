# Feather
[![build](https://api.travis-ci.com/caelunshun/feather.svg?branch=develop)](https://travis-ci.com/caelunshun/feather)
[![coverage](https://codecov.io/gh/caelunshun/feather/branch/develop/graph/badge.svg)](https://codecov.io/gh/caelunshun/feather)

An experimental Minecraft server implementation written in Rust.

This is only a hobby project, so don't expect it to go anywhere.

### Current Features
Feather is currently unusable for the most part. However, some basic features are already implemented:
- [x] Anvil world loading
- [x] Status ping
- [x] Players can see each other move
- [x] Tab list

Obviously, key features are missing. However, development is currently
quite active, and features should be added at a fast pace over the next few months.

### Running
If you want to try out the software, you have to compile Feather yourself.

Run ```cargo build --release``` in the repository's root directory to compile the code. To start the server,
you will have to do the following:
- Move the binary ```target/release/feather_server``` to your desired server directory.
- Copy the ```feather.toml``` from the project's root directory to the server directory.
- Copy a **1.13.2** Minecraft world save to the server directory.

Then, simply run the binary: ```./feather_server```.

The server currently only supports 1.13.2 clients and world saves. In the future, additional versions will be added.

**Note: the ```feather_blocks``` crate contains 130K lines of generated
code and can take up to an hour to compile in release mode. In addition,
it tends to take up around 8GiB RAM. In the future, we will provide
precompiled binaries so you don't have to wait for that.**
