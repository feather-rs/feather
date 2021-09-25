use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Item;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Item).add(EntityKind::Item);
}
