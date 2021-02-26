use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Minecart;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Minecart).add(EntityKind::Minecart);
}
