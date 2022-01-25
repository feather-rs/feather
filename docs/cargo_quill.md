# cargo-quill 
We have created an extension to cargo, that provides utilities for creating, 
building, and testing feather plugins written in rust. This utility is called
cargo-quill. 

The sourcecode for cargo-quill can be found in /feather/quill/cargo-quill. 
If there is a bug, or some missing feature, this is were to look.

# How to install:
Install cargo-quill by running the following command from the main folder. 
> cargo install --path quill/cargo-quill

## How to create a new plugin
This command works similar to 
> cargo-quill new 'name'

## build
> cargo-quill build 

Builds the source and puts a '.plugin' file in the target folder. Just like the
regular cargo build you can choose the compilation to be in '--release' mode or not. 
>  cargo-quill build --release

A command option you might not have considerd is that you can use '--native' if the
plugin should be compiled to native code instead of web assembly. 

> cargo-quill build --native 

You can also add some compression to the plugin.

> cargo-quill build --release --compression (0 to 9)
