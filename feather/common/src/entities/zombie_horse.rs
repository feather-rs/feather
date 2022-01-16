use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::ZombieHorse};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(ZombieHorse)
        .add(Health::new(15))
        .add(EntityKind::ZombieHorse);
}
