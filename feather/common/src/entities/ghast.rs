use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Ghast;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Ghast).add(EntityKind::Ghast);
}
