use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::CaveSpider};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(CaveSpider)
        .add(Health::new(12))
        .add(EntityKind::CaveSpider);
}
