use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Vindicator;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::VindicationIllager).with(Vindicator)
}
