# Bundles

Bundles are a convenient way of adding the required components for a given entity.
For instance, the Minecraft entity `Piglin` requires the marker `Piglin`, `Uuid`, `Position`, `Physics` and `Health`.

```rust
// Spawns a piglin at the given position with default values for `Uuid`, `Physics`, and `Health`
world.spawn(PiglinBundle::new(Position::new(0.0, 0.0, 0.0)));
```

- `PlayerBundle(Player, Uuid, Position, Health, Physics)`
- `ItemBundle(Position, Velocity, Physics)`
- `ZombieBundle`