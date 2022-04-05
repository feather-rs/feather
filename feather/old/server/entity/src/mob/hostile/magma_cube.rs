use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct MagmaCube;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::MagmaCube).with(MagmaCube)
}
