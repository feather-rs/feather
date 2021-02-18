use quill_common::entities::Villager;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Villager).add(EntityKind::Villager);
}
