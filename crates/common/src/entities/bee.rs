use quill_common::entities::Bee;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Bee).add(EntityKind::Bee);
}
