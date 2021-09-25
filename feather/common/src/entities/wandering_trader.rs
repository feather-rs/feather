use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::WanderingTrader;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(WanderingTrader)
        .add(EntityKind::WanderingTrader);
}
