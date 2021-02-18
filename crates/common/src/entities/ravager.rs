use quill_common::entities::Ravager;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Ravager).add(EntityKind::Ravager);
}
