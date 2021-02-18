use quill_common::entities::ShulkerBullet;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(ShulkerBullet).add(EntityKind::ShulkerBullet);
}
