use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::ZombieHorse;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(ZombieHorse).add(EntityKind::ZombieHorse);
}
