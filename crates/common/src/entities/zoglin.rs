use quill_common::entities::Zoglin;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Zoglin).add(EntityKind::Zoglin);
}
