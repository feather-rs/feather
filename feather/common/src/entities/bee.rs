use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Bee};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Bee).add(Health::new(10)).add(EntityKind::Bee);
}
