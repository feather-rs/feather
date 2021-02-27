use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Illusioner;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Illusioner).add(EntityKind::Illusioner);
}
