use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Zombie;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Zombie).add(EntityKind::Zombie);
}
