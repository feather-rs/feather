use quill_common::entities::Ghast;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Ghast).add(EntityKind::Ghast);
}
