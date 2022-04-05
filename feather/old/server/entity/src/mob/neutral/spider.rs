use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Spider;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Spider).with(Spider)
}
