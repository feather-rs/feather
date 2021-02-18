use quill_common::entities::Witch;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Witch).add(EntityKind::Witch);
}
