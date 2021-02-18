use quill_common::entities::ZombieHorse;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(ZombieHorse).add(EntityKind::ZombieHorse);
}
