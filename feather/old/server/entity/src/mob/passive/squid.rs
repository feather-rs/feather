use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Squid;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Squid).with(Squid)
}
