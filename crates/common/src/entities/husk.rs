use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Husk;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Husk).add(EntityKind::Husk);
}
