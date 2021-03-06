use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::LightningBolt;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(LightningBolt).add(EntityKind::LightningBolt);
}
