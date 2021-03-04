# feather-generated

Code generated from JSON files.

Code generators are written in Python and live in the `generators` directory. The script `regenerate.sh`
can be used to invoke all generators and regenerate all code.

Running these scripts requires `rustfmt` and Python 3.6 or greater. Note that code generation
is not a mandatory part of the build process, so you only need to regenerate code after modifying
a generator script.

`feather-generated` currently provides the following:
* Generated `Biome` enum and ID mappings.
* Generated `BlockKind` enum, which is used by `feather-blocks`.
* Generated `EntityKind` enum and ID mappings.
* Slot conversion functions for inventories, including the `Window` struct.
* Items and associated data (ID mappings, tool mappings, durability, ...)
* The `Particle` struct with protocol ID mappings.

Data is sourced from two sets of files:
* [`PrimsarineJS/minecraft-data`](https://github.com/PrismarineJS/minecraft-data), which provides the majority
of data. These files live in the `minecraft-data` subdirectory, which is a Git submodule. Make sure
that Git submodules are up to date before running the scripts.
* Feather's own data files, which live in the `feather` directory.
