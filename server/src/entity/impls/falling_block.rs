use shrev::ReaderId;
use specs::shrev::EventChannel;
use specs::{Builder, Component, DenseVecStorage, Entity, Read, ReadStorage, System, World, Write};

use feather_blocks::{Block, BlockExt};
use feather_core::packet::SpawnObject;
use feather_core::world::ChunkMap;

use crate::blocks::{BlockUpdateCause, BlockUpdateEvent};
use crate::entity::component::{PacketCreatorComponent, SerializerComponent};
use crate::entity::metadata::Metadata;
use crate::entity::movement::degrees_to_stops;
use crate::entity::{EntityDestroyEvent, PositionComponent, VelocityComponent};
use crate::physics::{EntityPhysicsLandEvent, PhysicsBuilder};
use crate::util::protocol_velocity;
use crate::world_ext::WorldExt;
use feather_core::{Packet, Position};
use specs::world::LazyBuilder;
use uuid::Uuid;

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

pub fn create<'a>(builder: LazyBuilder<'a>, position: Position, block: Block) -> LazyBuilder<'a> {
    let meta = {
        let mut meta_falling_block = crate::entity::metadata::FallingBlock::default();
        meta_falling_block.set_spawn_position(position.block_pos());
        Metadata::FallingBlock(meta_falling_block)
    };

    builder
        .with(FallingBlockComponent { block })
        .with(
            PhysicsBuilder::new()
                .gravity(-0.04)
                .drag(0.98)
                .bbox(0.98, 0.98, 0.98)
                .build(),
        )
        .with(meta)
        .with(PacketCreatorComponent(&create_packet))
    //.with(SerializerComponent(&serialize)) TODO
}

fn create_packet(world: &World, entity: Entity) -> Box<dyn Packet> {
    let block = world
        .get::<FallingBlockComponent>(entity)
        .block
        .native_state_id();
    let position = world.get::<PositionComponent>(entity).current;
    let (velocity_x, velocity_y, velocity_z) =
        protocol_velocity(world.get::<VelocityComponent>(entity).0);

    let packet = SpawnObject {
        entity_id: entity.id() as i32,
        object_uuid: Uuid::new_v4(),
        ty: 70,
        x: position.x,
        y: position.y,
        z: position.z,
        pitch: degrees_to_stops(position.pitch),
        yaw: degrees_to_stops(position.yaw),
        data: i32::from(block),
        velocity_x,
        velocity_y,
        velocity_z,
    };

    Box::new(packet)
}
