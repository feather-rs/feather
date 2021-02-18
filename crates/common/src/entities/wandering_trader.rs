use quill_common::entities::WanderingTrader;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(WanderingTrader).add(EntityKind::WanderingTrader);
}
