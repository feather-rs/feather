use crate::Game;
use feather_core::anvil::block_entity::{BlockEntityData, BlockEntityVariant};
use feather_core::{
    anvil::entity::{EntityData, EntityDataKind},
    blocks::BlockKind,
    util::BlockPosition,
};
use fecs::{Entity, EntityBuilder, World};

pub type BumpVec<'bump, T> = bumpalo::collections::Vec<'bump, T>;

pub trait EntityLoaderFn:
    Fn(EntityData) -> anyhow::Result<EntityBuilder> + Send + Sync + 'static
{
}

impl<F> EntityLoaderFn for F where
    F: Fn(EntityData) -> anyhow::Result<EntityBuilder> + Send + Sync + 'static
{
}

pub trait BlockEntityLoaderFn:
    Fn(BlockEntityData) -> anyhow::Result<EntityBuilder> + Send + Sync + 'static
{
}

impl<F> BlockEntityLoaderFn for F where
    F: Fn(BlockEntityData) -> anyhow::Result<EntityBuilder> + Send + Sync + 'static
{
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Weather {
    Clear,
    Rain,
    Thunder,
}

/// A registration for a function to convert an `EntityData`
/// to an `EntityBuilder` for spawning into the world. The
/// registration must provide the `EntityDataKind` it handles
/// to determine which `EntityData`s to pass to this function.
pub struct EntityLoaderRegistration {
    /// The loader function.
    pub f: &'static dyn EntityLoaderFn,
    /// The kind of `EntityData` which this loader
    /// function will accept.
    pub kind: EntityDataKind,
}

impl EntityLoaderRegistration {
    pub fn new(kind: EntityDataKind, f: &'static dyn EntityLoaderFn) -> Self {
        Self { f, kind }
    }
}

inventory::collect!(EntityLoaderRegistration);

/// Same as `EntityLoaderRegistration`, but for block entities.
pub struct BlockEntityLoaderRegistration {
    pub f: &'static dyn BlockEntityLoaderFn,
    pub kind: BlockEntityVariant,
}

inventory::collect!(BlockEntityLoaderRegistration);

/// Handles interactions (Use Item key) with a block.
pub trait InteractionHandler: Send + Sync {
    /// Called whenever a player right clicks on the block
    fn handle_interaction(
        &self,
        game: &mut Game,
        world: &mut World,
        pos: BlockPosition,
        player: Entity,
        window_id: u8,
    );

    /// Returns the kind of block handled by this handler.
    fn block_kind(&self) -> BlockKind;
}

inventory::collect!(Box<dyn InteractionHandler>);

/// Wrapper around the send/receive channels which will be used to
/// notify server thread of shutdown due to ctrl+C or /stop command.
pub struct ShutdownChannels {
    pub tx: crossbeam::channel::Sender<()>,
    pub rx: crossbeam::channel::Receiver<()>,
}

impl ShutdownChannels {
    pub fn new() -> Self {
        let (tx, rx) = crossbeam::bounded(1);
        Self { tx, rx }
    }
}

impl Default for ShutdownChannels {
    fn default() -> Self {
        Self::new()
    }
}
