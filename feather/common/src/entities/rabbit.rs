use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Rabbit;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Rabbit).add(EntityKind::Rabbit);
}
