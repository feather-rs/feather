use anyhow::bail;
use libcraft::{EntityKind, ProfileProperty};
use quill::{
    components::{CreativeFlying, EntityKindComponent, Sneaking, Sprinting},
    entities::Player,
};
use vane::{Component, EntityBuilder, SysResult};

#[derive(Debug)]
pub struct PlayerProfile(pub Vec<ProfileProperty>);

impl Component for PlayerProfile {}

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(Player)
        .add(CreativeFlying(false))
        .add(Sneaking(false))
        .add(Sprinting(false))
        .add(EntityKindComponent(EntityKind::Player));
}

/// The hotbar slot a player's cursor is currently on
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct HotbarSlot(usize);

impl Component for HotbarSlot {}

impl HotbarSlot {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn get(&self) -> usize {
        self.0
    }

    pub fn set(&mut self, id: usize) -> SysResult {
        if id > 8 {
            bail!("invalid hotbar slot id");
        }

        self.0 = id;
        Ok(())
    }
}
