use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Phantom;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Phantom).with(Phantom)
}
