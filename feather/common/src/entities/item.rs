use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Item};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Item).add(Health::new(5)).add(EntityKind::Item);
}
