use quill_common::entities::EndCrystal;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(EndCrystal).add(EntityKind::EndCrystal);
}
