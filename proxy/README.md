A simple proxy for use during development. It proxies a connection between
client and server and prints out packets going over the network.

This tool is useful when figuring out how Minecraft implements features
over the protocol.

#### Usage

To set up the proxy with a vanilla server and client:

* Start the vanilla 1.16.5 server. Make sure to set `online-mode=false`
  or the proxy will not work for the time being.
* Run the proxy with `cargo run --bin proxy -- --proxy-address 127.0.0.1:25577 --server-address 127.0.0.1:25565`.
* Connect your client to `localhost:25577`.

To set up the proxy with feather server:

* Build and start feather (See main [`README.md`](../../README.md))
* In the `config.toml` set `online_mode = false`.
* Run the proxy with `cargo run --bin proxy -- --proxy-address 127.0.0.1:25577 --server-address 127.0.0.1:25565`.
* Connect your client to `localhost:25577`.

The proxy supports multiple client connections, in case you need
to debug protocol semantics with multiple players.
