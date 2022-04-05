use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct IronGolem;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::IronGolem).with(IronGolem)
}
