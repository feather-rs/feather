use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Donkey;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Donkey).add(EntityKind::Donkey);
}
