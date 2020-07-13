`feather-server` and its subcrates, implementing a server on top of `feather-core`.

### Subcrates

Please see [the book](https://feather-rs.github.io/book) to find out in which crate new features belong. If you're
not sure where to put something, feel free to ask on our Discord.

The philosophy here is to have many small crates to enforce modularity. If a crate starts getting
too large, it will be split into subcrates.

To create a new crate, please copy the `template` directory and update the package name
for the new crate.

Note that all crates should have `#![forbid(unsafe_code)]` at the crate root unless it is made
explicit here that a crate contains unsafe code.

* `types`: all components and resources which subcrates would like to make available to other subcrates.
Acts somewhat like a more elegant C/C++ header file.
* `util`: small utility functions as well as trivial game logic which doesn't need to be in its own crate (e.g. world time)
* `entity`: entity implementations  (items, arrows, falling blocks, mobs, ...). UNSAFE: used for item collection in `object::item::item_collect:system`.
* `block`: block entity implementations (chests, furnaces, command blocks, ...)
* `player`: logic pertaining directly to players, e.g. chunk sending, chat, the view system. Also contains all packet handlers.
* `network`: the TCP listener and IO worker implementation for communication with clients
* `config`: the configuration file and struct, plus loading/saving logic
* `chunk`: the chunk worker and chunk loading/saving logic
* `physics`: physics systems, including entity and (soon) fluid mechanics
* `lighting`: block and sky lighting
* `packet_buffer`: various data structures to buffer packets between the IO worker and the server threads
* `weather`: weather handling, scheduling

