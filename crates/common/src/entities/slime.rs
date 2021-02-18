use quill_common::entities::Slime;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Slime).add(EntityKind::Slime);
}
