# libcraft

General-purpose Minecraft types and functions for Rust. Work in progress; code is being moved from [Feather](../README.md).

`libcraft` is part of the Feather project, but it aims to provide standalone functionality for use in Minecraft-related tools
like map editors, world converters, etc.

Once finished, this crate will provide:
* Block struct with access to properties, block state values, and IDs
* Item struct with access to properties and IDs
* Inventory and [window](https://wiki.vg/Inventory) definitions
* An implementation of Minecraft's [in-memory chunk data structure](https://wiki.vg/Chunk_Format)
* The [JSON Text/ChatComponent API](https://wiki.vg/Chat)
* Region file loading
* Packets from the [protocol](https://wiki.vg/Protocol)

Each piece of functionality is in its own crate. All `libcraft-*` crates are reexported from the main `libcraft` crate.

## License
Copyright 2021 Caelum van Ispelen

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
