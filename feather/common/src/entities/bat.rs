use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Bat};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Bat).add(Health::new(6)).add(EntityKind::Bat);
}
