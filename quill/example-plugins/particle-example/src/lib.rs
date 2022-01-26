use quill::{Game, Particle, ParticleKind, Plugin, Position};

#[quill::plugin]
struct ParticleExample {}

impl Plugin for ParticleExample {
    fn enable(_game: &mut quill::Game, setup: &mut quill::Setup<Self>) -> Self {
        setup.add_system(particle_system);

        ParticleExample {}
    }

    fn disable(self, _game: &mut quill::Game) {}
}

fn particle_system(_plugin: &mut ParticleExample, game: &mut Game) {
    let mut position = Position {
        x: 0.0,
        y: 65.0,
        z: 0.0,
        pitch: 0.0,
        yaw: 0.0,
    };

    let particle = Particle {
        kind: ParticleKind::SoulFireFlame,
        offset_x: 0.0,
        offset_y: 0.0,
        offset_z: 0.0,
        count: 1,
    };

    game.spawn_particle(position, particle);

    position.x += 1.0;

    let particle2 = Particle {
        kind: ParticleKind::Dust {
            red: 1.0,
            green: 1.0,
            blue: 0.0,
            scale: 3.5,
        },
        offset_x: 0.0,
        offset_y: 0.0,
        offset_z: 0.0,
        count: 1,
    };

    // Initialise an empty ecs-entity builder
    let mut builder = game.create_empty_entity_builder();

    // Add the required components to display a particle effect
    builder.add(position);
    builder.add(particle2);

    // Finish the builder, this will spawn the ecs-entity in the ecs-world
    builder.finish();
}
