use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::MagmaCube;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(MagmaCube).add(EntityKind::MagmaCube);
}
