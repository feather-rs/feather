use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Cod};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Cod).add(Health::new(3)).add(EntityKind::Cod);
}
