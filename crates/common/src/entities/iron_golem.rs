use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::IronGolem;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(IronGolem).add(EntityKind::IronGolem);
}
