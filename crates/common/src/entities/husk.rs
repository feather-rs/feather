use quill_common::entities::Husk;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Husk).add(EntityKind::Husk);
}
