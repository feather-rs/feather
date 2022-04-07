// This file is @generated. Please do not edit.
use base::EntityKind;
use quill::entities::ZombieHorse;
use vane::EntityBuilder;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(ZombieHorse).add(EntityKind::ZombieHorse);
}
