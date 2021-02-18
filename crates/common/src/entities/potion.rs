use quill_common::entities::Potion;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Potion).add(EntityKind::Potion);
}
