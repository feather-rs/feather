### minecraft-proxy

A simple proxy for use during development. It proxies a connection between
client and server and prints out packets going over the network.

This tool is useful when figuring out how Minecraft implements features
over the protocol.

#### Usage

To set up the proxy with a vanilla server and client:

* Start the vanilla 1.16.3 server. Make sure to set `network-compression-threshold=-1` and `online-mode=false`
or the proxy will not work for the time being.
* Run the proxy with `cargo run --bin minecraft-proxy -- --port 25566 --server-port 25565`
* Connect your client to `localhost:25566`

The proxy supports multiple client connections, in case you need
to debug protocol semantics with multiple players.
