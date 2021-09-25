use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::FishingBobber;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(FishingBobber).add(EntityKind::FishingBobber);
}
