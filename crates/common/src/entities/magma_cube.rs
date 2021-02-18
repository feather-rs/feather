use quill_common::entities::MagmaCube;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(MagmaCube).add(EntityKind::MagmaCube);
}
