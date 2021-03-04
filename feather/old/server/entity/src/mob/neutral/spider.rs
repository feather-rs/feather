use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Spider;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Spider).with(Spider)
}
