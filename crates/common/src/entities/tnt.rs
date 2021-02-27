use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Tnt;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Tnt).add(EntityKind::Tnt);
}
