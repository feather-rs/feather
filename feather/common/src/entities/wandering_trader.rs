use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::WanderingTrader};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(WanderingTrader)
        .add(Health::new(20))
        .add(EntityKind::WanderingTrader);
}
