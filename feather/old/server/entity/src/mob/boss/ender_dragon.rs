use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct EnderDragon;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::EnderDragon).with(EnderDragon)
}
