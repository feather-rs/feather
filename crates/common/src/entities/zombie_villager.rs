use quill_common::entities::ZombieVillager;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(ZombieVillager).add(EntityKind::ZombieVillager);
}
