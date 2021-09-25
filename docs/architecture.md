### Architecture

Feather uses the Entity-Component-System architecture, also known as ECS. This architecture
is widely used in the Rust gamedev ecosystem. 

In the ECS architecture, there are three key types of objects:
* Entities: these are just IDs. In Feather, these are represented by the `Entity` struct.
They allow access to components.
* Components: these represent entities' data. Each entity can have zero or one component of every type. For example, `Position`
stores an entity position, and entities with the `Position` component have a position. You can access components
via `Game.ecs.get::<T>()`, where `T` is the component you want.
* Systems: functions that run each tick. While components are data, systems are logic. They operate on components.

ECS implementations allow for _queries_ that allow iteration over all entities with a specific set of components.
For example, to implement trivial physics:

```rust
for (entity, (position, velocity)) in game.ecs.query::<(&mut Position, &Velocity)>().iter() {
    *position += *velocity;
}
```

The above code snippet iterates over _all_ entities with `Position` and `Velocity` components.

For more information on the ECS, we recommend checking out the [`hecs`](https://docs.rs/hecs) documentation.

The Feather game state is defined in the `Game` struct, which lives in `crates/common/src/game.rs`.
This struct contains the `World` (blocks) and the `Ecs` (entities). It also provides
methods for common tasks, like "spawn entity" or "remove entity" or "get block."

Note that entities in the ECS correspond either to Minecraft entities, like players or zombies,
or to internal entities like the "console entity." In general, you don't have to worry about
this distinction.

### Commonly used components

This is a list of components that are frequently accessed throughout the codebase.
* `Position`
* `Gamemode` for players
* `Name` - player's username (not for other entities)
* `CustomName` - entity's custom name (not for players)
* `Inventory`
* `Window` - wraps one or more `Inventory`s that the player is looking at right now. In a chest,
for example, a player's window would wrap the player inventory and the chest inventory.

### Crate Structure

Feather is a complex codebase with multiple components. To improve modularity and reusability, we've
split the codebase into a series of crates:

* Core Minecraft functionality (not specific to Feather) goes in [`libcraft`](https://github.com/feather-rs/libcraft).
For example, the block, item, chunk, and region file structs live in `libcraft`. `libcraft` code is intended
for use in other Rust Minecraft tools, like map editors or world file converters.
* The plugin API lives in [`quill`](https://github.com/feather-rs/quill), which actually consists of three major crates:
  * `quill-common` is shared between Feather itself and plugins. This is where most of our ECS components are defined,
  so that both plugins and Feather can access them.
  * `quill-sys` provides FFI functions for "host calls." Host calls are low-level functions that
  plugins call to perform actions. For example, "get component" and "send message" are host calls.
  * `quill` is the public-facing plugin API. It reexports types from `quill-common` and wraps the FFI functions in `quill-sys`
  with a safe, idiomatic API.
* The remainder of the code is in Feather itself, which consists of three major crates:
  * `feather-common` implements gameplay: it defines ECS systems that run the Minecraft game. For example, it includes
  physics, block placement/digging, chat, etc. It operates on the types defined in `libcraft` and `quill-common`.
  * `feather-server` is a wrapper around `feather-common` that provides packet handling and sending.
  * `feather-plugin-host` implements the FFI functions in `quill-sys`.

### Adding a component

Components should be defined in the `quill-common` crate so plugins can access them. 
(Some components can also be exported from `libcraft` if they're reusable outside of Feather.) 
Just define a struct for the component, derive `Serialize` and `Deserialize`, and add
it to `components.rs`.

The component can then be accessed both from Feather and from plugins.

### Events

In Feather, events are components. An entity with the `PlayerJoinEvent` component just joined
the game, for example.

The event sytsem serves as a mechanism to communicate between different crates and modules.
For example, triggering a `BlockChangeEvent` _anywhere_ causes `feather-server` to send block
update packets to players.

To trigger an event, use `game.ecs.insert_entity_event(entity, event)`. `entity` should be the
entity that the event happened to, e.g. for `PlayerJoinEvent`, it's the player that joined.

Alternatively, if the event is not related to a specific entity, call `game.ecs.insert_event(event)`.
For example, `BlockChangeEvent` is one such event.

To handle events, query for entities with that component. For example, to query
for players that just joined, use:

```rust
for (player_entity, event) in game.ecs.query::<(&PlayerJoinEvent)>().iter() {
    // handle event...
}
```

### Sending packets

Most features need to send packets to clients. The Minecraft protocol and its
packets are documented at [wiki.vg](https://wiki.vg/Protocol).

In Feather, the `Client` struct in `crates/server/src/client.rs` encapsulates
the packet sending code. Add a method there to send the packet you need.

It's not possible to send packets from the `feather-common` crate. Instead, you should
trigger an event and handle it in `feather-server`. 

### Sending packets to nearby players

Some packets should be sent to all players that can see a given entity, or all
players that can see a given block. Use `Server::broadcast_nearby_with` for this.

### Receiving packets

Packets are handled in `crates/server/src/packet_handlers.rs`. Add the necessary match
arm and implement your packet handler.

### Further questions

This documentation is a work in progress. Contact us on Discord if you have further
questions!
