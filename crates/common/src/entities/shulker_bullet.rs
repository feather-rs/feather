use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::ShulkerBullet;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(ShulkerBullet).add(EntityKind::ShulkerBullet);
}
