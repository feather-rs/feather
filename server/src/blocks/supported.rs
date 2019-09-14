//! Breaking of blocks which require solid supports underneath.

use crate::blocks::{BlockNotifyEvent, BlockUpdateCause, BlockUpdateEvent};
use feather_blocks::{Block, BlockExt};
use feather_core::world::ChunkMap;
use shrev::EventChannel;
use specs::{Read, ReaderId, System, Write};

/// System to break a non-solid block when the block
/// underneath it is destroyed.
///
/// This system listens to `BlockNotifyEvent`s.
#[derive(Default)]
pub struct SupportBlockBreakSystem {
    reader: Option<ReaderId<BlockNotifyEvent>>,
}

impl<'a> System<'a> for SupportBlockBreakSystem {
    type SystemData = (
        Write<'a, ChunkMap>,
        Write<'a, EventChannel<BlockUpdateEvent>>,
        Read<'a, EventChannel<BlockNotifyEvent>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut chunk_map, mut block_updates, block_notifies) = data;

        for event in block_notifies.read(self.reader.as_mut().unwrap()) {
            if event.block.is_solid() {
                // Nothing to do.
                continue;
            }

            if !(event.pos.y - event.notified_by.y == 1
                && event.pos.x == event.notified_by.x
                && event.pos.z == event.notified_by.z)
            {
                continue;
            }

            let below = chunk_map.block_at(event.notified_by).unwrap_or(Block::Air);

            if !below.is_solid() {
                let _ = chunk_map.set_block_at(event.pos, Block::Air);
            }

            // TODO: drop item

            let update = BlockUpdateEvent {
                cause: BlockUpdateCause::SupportDestroyed,
                pos: event.pos,
                old_block: event.block,
                new_block: Block::Air,
            };
            block_updates.single_write(update);
        }
    }

    setup_impl!(reader);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testframework as t;
    use feather_core::BlockPosition;
    use feather_core::{Chunk, ChunkPosition};
    use specs::WorldExt;

    #[test]
    fn test_support_block_break_system() {
        let (mut world, mut dispatcher) = t::builder()
            .with(SupportBlockBreakSystem::default(), "")
            .build();

        {
            let mut chunk_map = world.fetch_mut::<ChunkMap>();
            let mut chunk = Chunk::default();

            chunk.set_block_at(0, 0, 0, Block::Air);
            chunk.set_block_at(0, 1, 0, Block::Grass);
            chunk_map.set_chunk_at(ChunkPosition::new(0, 0), chunk);
        }

        let event = BlockNotifyEvent {
            block: Block::Grass,
            pos: BlockPosition::new(0, 1, 0),
            notified_by: BlockPosition::new(0, 0, 0),
        };
        t::trigger_event(&world, event);

        dispatcher.dispatch(&world);
        world.maintain();

        assert_eq!(
            world
                .fetch::<ChunkMap>()
                .block_at(BlockPosition::new(0, 1, 0))
                .unwrap(),
            Block::Air
        );
    }
}
