use shrev::ReaderId;
use specs::shrev::EventChannel;
use specs::{Component, DenseVecStorage, Read, ReadStorage, System, Write};

use feather_blocks::Block;
use feather_core::world::ChunkMap;

use crate::blocks::{BlockUpdateCause, BlockUpdateEvent};
use crate::entity::{EntityDestroyEvent, EntityType};
use crate::physics::EntityPhysicsLandEvent;

/// Component for falling block entities.
pub struct FallingBlockComponent {
    pub block: Block,
}

impl Default for FallingBlockComponent {
    fn default() -> Self {
        FallingBlockComponent {
            block: Block::Stone,
        }
    }
}

impl Component for FallingBlockComponent {
    type Storage = DenseVecStorage<Self>;
}

/// This system listens to `EntityPhysicsLandEvent`s.
#[derive(Default)]
pub struct FallingBlockLandSystem {
    reader: Option<ReaderId<EntityPhysicsLandEvent>>,
}

/// System for handling when a falling block lands
/// on the ground, destroying the entity and setting the block.
impl<'a> System<'a> for FallingBlockLandSystem {
    type SystemData = (
        Read<'a, EventChannel<EntityPhysicsLandEvent>>,
        ReadStorage<'a, FallingBlockComponent>,
        ReadStorage<'a, EntityType>,
        Write<'a, EventChannel<EntityDestroyEvent>>,
        Write<'a, EventChannel<BlockUpdateEvent>>,
        Write<'a, ChunkMap>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (events, falling_blocks, types, mut destroy_events, mut block_updates, mut chunk_map) =
            data;

        // Process events
        for event in events.read(&mut self.reader.as_mut().unwrap()) {
            let entity = event.entity;

            let entity_type = types.get(entity).unwrap();
            if *entity_type != EntityType::FallingBlock {
                return;
            }
            let falling_block = falling_blocks.get(entity).unwrap();

            let destroy_event = EntityDestroyEvent { entity };
            destroy_events.single_write(destroy_event);

            let pos = event.pos.block_pos();
            let old_block = chunk_map.block_at(pos).unwrap();
            chunk_map.set_block_at(pos, falling_block.block).unwrap();

            let update_event = BlockUpdateEvent {
                cause: BlockUpdateCause::FallingBlock,
                pos,
                old_block,
                new_block: falling_block.block,
            };

            block_updates.single_write(update_event);
        }
    }

    setup_impl!(reader);
}
