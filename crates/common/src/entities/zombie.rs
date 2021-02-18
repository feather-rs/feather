use quill_common::entities::Zombie;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Zombie).add(EntityKind::Zombie);
}
