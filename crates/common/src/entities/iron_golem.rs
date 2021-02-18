use quill_common::entities::IronGolem;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(IronGolem).add(EntityKind::IronGolem);
}
