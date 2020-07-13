use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct IronGolem;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::IronGolem).with(IronGolem)
}
