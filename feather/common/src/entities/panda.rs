use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Panda;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Panda).add(EntityKind::Panda);
}
