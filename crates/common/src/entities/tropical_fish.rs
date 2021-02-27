use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::TropicalFish;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(TropicalFish).add(EntityKind::TropicalFish);
}
