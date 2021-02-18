use quill_common::entities::Ocelot;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Ocelot).add(EntityKind::Ocelot);
}
