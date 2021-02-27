use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Squid;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Squid).add(EntityKind::Squid);
}
