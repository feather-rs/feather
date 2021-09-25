use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::CaveSpider;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(CaveSpider).add(EntityKind::CaveSpider);
}
