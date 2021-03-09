### minecraft-proxy

A simple proxy for use during development. It proxies a connection between
client and server and prints out packets going over the network.

This tool is useful when figuring out how Minecraft implements features
over the protocol.

NOTE: some packets aren't fully implemented yet and thus will either only print a list of bytes or in the case of `ChunkData` and `UpdateLight` crash the proxy. (Feel free to contribute 😊)

#### Usage

To set up the proxy with a vanilla server and client:

* Start the vanilla 1.16.3 server. Make sure to set `network-compression-threshold=-1` and `online-mode=false`
or the proxy will not work for the time being.
* Run the proxy with `cargo run --bin minecraft-proxy -- --port 25577 --server-port 25565`.
* Connect your client to `localhost:25577`.

To set up the proxy with feather server:

* Build and start feather (See main [`README.md`](../../README.md))
* In the `config.toml` set `online_mode = false` and `compression_threshold = -1`.
* Run the proxy with `cargo run --bin minecraft-proxy -- --port 25577 --server-port 25565`.
* Connect your client to `localhost:25577`.

The proxy supports multiple client connections, in case you need
to debug protocol semantics with multiple players.
