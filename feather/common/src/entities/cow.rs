use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Cow};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Cow).add(Health::new(10)).add(EntityKind::Cow);
}
