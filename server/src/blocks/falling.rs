use shrev::ReaderId;
use specs::shrev::EventChannel;
use specs::{Read, System, Write};

use feather_core::world::ChunkMap;

use feather_blocks::{Block, BlockExt};

use crate::blocks::{BlockNotifyEvent, BlockUpdateCause, BlockUpdateEvent};

/// This system listens to `BlockNotifyEvent`s.
#[derive(Default)]
pub struct FallingBlockCreationSystem {
    reader: Option<ReaderId<BlockNotifyEvent>>,
}

impl<'a> System<'a> for FallingBlockCreationSystem {
    type SystemData = (
        Read<'a, EventChannel<BlockNotifyEvent>>,
        Write<'a, EventChannel<BlockUpdateEvent>>,
        Write<'a, ChunkMap>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (events, mut block_update, mut chunk_map) = data;

        // Process events
        for event in events.read(&mut self.reader.as_mut().unwrap()) {
            match event.block {
                Block::Sand | Block::RedSand | Block::Gravel => {
                    let mut below = event.pos;
                    below.y -= 1;

                    if !chunk_map.block_at(below).unwrap_or(Block::Air).is_solid() {
                        chunk_map.set_block_at(event.pos, Block::Air).unwrap();

                        let event = BlockUpdateEvent {
                            cause: BlockUpdateCause::FallingBlock,
                            pos: event.pos,
                            old_block: event.block,
                            new_block: Block::Air,
                        };

                        block_update.single_write(event);

                        debug!("TODO: Spawn falling-block entity.")
                    }
                }
                _ => (),
            }
        }
    }

    setup_impl!(reader);
}
