use quill_common::entities::Fireball;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Fireball).add(EntityKind::Fireball);
}
