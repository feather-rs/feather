# Particles
Quill can spawn a particle using the function provided in `quill::Game` or by manually spawning an ecs-entity with a `Position` and `Particle` component.

Spawning a flame particle at `0, 0, 0` is simple:
```rust
let position = Position {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        pitch: 0.0,
        yaw: 0.0,
    };

// Create the flame particle without any offset and a count of 1
let particle = Particle {
    kind: ParticleKind::Flame,
    offset_x: 0.0,
    offset_y: 0.0,
    offset_z: 0.0,
    count: 1,
}

// Spawn the particle in the world
game.spawn_particle(position, particle);
```

Manually spawn a flame particle using the ecs system:
```rust
let position = Position {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        pitch: 0.0,
        yaw: 0.0,
    };

let particle = Particle {
        kind: ParticleKind::Flame,
        offset_x: 0.0,
        offset_y: 0.0,
        offset_z: 0.0,
        count: 1,
    };

// Initialise an empty ecs-entity builder
let mut builder = game.create_empty_entity_builder();

// Add the required components to display a particle effect
builder.add(position);
builder.add(particle);

// Finish the builder, this will spawn the ecs-entity in the ecs-world
builder.finish();
```
This approach allows you to add more components, to the entity which can be useful!

Creating particles with extra data is also simple:
```rust
let dust_particle = Particle {
    kind: ParticleKind::Dust {
        red: 1.0,
        green: 0.0,
        blue: 1.0,
        scale: 1.0
    },
    offset_x: 0.0,
    offset_y: 0.0,
    offset_z: 0.0
}
```


A full example can be found in the example-plugins directory under `particle-example`