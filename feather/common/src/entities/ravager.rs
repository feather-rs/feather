use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Ravager;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Ravager).add(EntityKind::Ravager);
}
