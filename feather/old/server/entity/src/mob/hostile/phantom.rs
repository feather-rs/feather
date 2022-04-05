use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Phantom;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Phantom).with(Phantom)
}
