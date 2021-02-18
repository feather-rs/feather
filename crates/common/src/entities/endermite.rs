use quill_common::entities::Endermite;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Endermite).add(EntityKind::Endermite);
}
