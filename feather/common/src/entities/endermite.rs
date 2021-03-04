use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Endermite;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Endermite).add(EntityKind::Endermite);
}
