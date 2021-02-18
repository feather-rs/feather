use quill_common::entities::Minecart;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Minecart).add(EntityKind::Minecart);
}
