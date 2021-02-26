use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Bee;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Bee).add(EntityKind::Bee);
}
