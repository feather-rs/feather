use quill_common::entities::Illusioner;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Illusioner).add(EntityKind::Illusioner);
}
