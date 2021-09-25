use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Zoglin;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Zoglin).add(EntityKind::Zoglin);
}
