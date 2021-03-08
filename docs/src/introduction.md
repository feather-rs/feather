# Introduction

Feather is split into several smaller crates and projects:
- `libcraft` contains data related to Minecraft and which is not directly related to feather.
    - `blocks` contains all data related to Minecraft blocks. 
    - `core` reexports all subcrates and contains data related to Minecraft which is not covered by any other subcrate.
    - `generators` is used to generate `.rs` source files from `prismarinejs/minecraft-data`.
    - `items` contains all data related to Minecraft items.
    - `macros` contains all proc-macros used throughout `libcraft`.
    - `particles` should maybe be moved to core, since it is a single file?
- `feather`
    - `base` is deprecated in favour of `libcraft`
    - `blocks` is deprecated in favour of `libcraft`
    - `common`
    - `datapacks` should maybe be deprecated in favour of `libcraft` and be based on `prismarinejs/minecraft-data`.
    - `ecs`
    - `generated` is deprecated in favour of `libcraft`
    - `plugin-host`
    - `protocol`
    - `server`
    - `utils`
    - `worldgen`
- `quill` is the interface between feather and plugins, it itself consist of multiple smaller crates.
    - `api` is what plugins will inteface with.
    - `cargo-quill` is a CLI which can be used to build, run, and publish plugins.
    - `common` is for what both `quill-api` and `feather` needs access to.
    - `sys` is where all externally exported functions are defined.
    - `sys-macros` contains a procmacro which redefines host calls depending on what architecture is being targeted.