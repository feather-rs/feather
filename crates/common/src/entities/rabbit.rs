use quill_common::entities::Rabbit;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Rabbit).add(EntityKind::Rabbit);
}
