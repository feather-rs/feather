# Feather - Quill - Plugins
Feather has support for plugins. They can either be run as native code, or through WebAssembly using [Wasmer](https://github.com/wasmerio/wasmer). The API for the two options are the same, but wasmer has a few benefits for the cost of slightly worse performance. ([More details](#Wasm-or-Native)).

## Getting Started

# Building and Running
To build a quill plugin you will need a tool called [cargo-quill](https://github.com/¨-rs/feather/tree/main/quill/cargo-quill). To install cargo quill you need the [rust build tools](https://www.rust-lang.org/tools/install), and clone Feather's [git repository](https://github.com/feather-rs/feather).

Once that is setup, locate the folder `feather/quill/cargo-quill` in a terminal. Then run the following command.

```bash
cargo install --path <path to quill>
```
This adds `cargo-quill` to path, which can be tested by running.
```bash
cargo-quill --help
```

To create a plugin run the following command.
```bash
cargo-quill new <name>
```

This creates a simple test plugin called <name>. To build this example plugin head into the directory of the plugin (with the Cargo.toml in it) and run:

```bash
cargo-quill build
```


By default, the plugin will be built in `debug` mode. The build process will be fast, but the plugin itself will be slower. By specifying the `--release` flag, you can build the plugin in `release` mode, which will take longer, but results in a faster plugin. Without any other flags, the plugin will be compiled to a wasm plugin. If you want to compile it to native code, add the `--native` flag.


Once finished the command will have created a file ending in '.plugin'. It is placed in either `/target/release` or `/target/debug` depending on if the release flag is provided. 

To test the plugin, a feather server binary is needed. Inside the folder where the binary is located, create a folder `plugins` (if it doesn't already exist). Put the '.plugin' there and run the server.



## Wasm or Native? 

### Wasm
Wasm comes with the added benefit of running in a "sandbox", which means that if a plugin at any points crashes it won't take down whole server immediately. The server can try to restart it, but if all else fails shuts down the server gracefully.

Wasm plugins manage their own memory, and don't directly share the same memory as the rest of the server, what system it was compiled on doesn't affect anything either.

Since Wasm is designed to only do computation it doesn't have any IO capabilities in normal cases, this means plugins compiled to Wasm won't be able to do any logging or save/load configuration files by their own means. They are limited to use the api that Feather provides.

### Native
Native comes with an advantage when it comes to performance, it was built to run on your specific system and needs to be compiled for it.
It comes with a downside though, if a native plugin crashes the entire server crashes.
