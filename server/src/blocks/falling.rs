use shrev::ReaderId;
use specs::shrev::EventChannel;
use specs::{Read, System, Write};

use feather_core::world::ChunkMap;

use feather_blocks::{Block, BlockExt};

use crate::blocks::{BlockNotifyEvent, BlockUpdateCause, BlockUpdateEvent};
use crate::util::Util;
use feather_core::Position;

/// This system listens to `BlockNotifyEvent`s.
#[derive(Default)]
pub struct FallingBlockCreationSystem {
    reader: Option<ReaderId<BlockNotifyEvent>>,
}

impl<'a> System<'a> for FallingBlockCreationSystem {
    type SystemData = (
        Read<'a, EventChannel<BlockNotifyEvent>>,
        Read<'a, Util>,
        Write<'a, EventChannel<BlockUpdateEvent>>,
        Write<'a, ChunkMap>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (events, util, mut block_update, mut chunk_map) = data;

        // Process events
        for event in events.read(&mut self.reader.as_mut().unwrap()) {
            match event.block {
                Block::Sand | Block::RedSand | Block::Gravel => {
                    let mut below = event.pos;
                    below.y -= 1;

                    if !chunk_map.block_at(below).unwrap_or(Block::Air).is_solid() {
                        chunk_map.set_block_at(event.pos, Block::Air).unwrap();

                        let update_event = BlockUpdateEvent {
                            cause: BlockUpdateCause::FallingBlock,
                            pos: event.pos,
                            old_block: event.block,
                            new_block: Block::Air,
                        };

                        block_update.single_write(update_event);

                        let mut entity_pos: Position = event.pos.world_pos();
                        entity_pos.x += 0.49;
                        entity_pos.z += 0.49;

                        util.spawn_falling_block(entity_pos, glm::vec3(0.0, 0.0, 0.0), event.block)
                    }
                }
                _ => (),
            }
        }
    }

    setup_impl!(reader);
}
