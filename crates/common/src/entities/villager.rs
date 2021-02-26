use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Villager;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Villager).add(EntityKind::Villager);
}
