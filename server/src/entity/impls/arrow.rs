use shrev::EventChannel;
use specs::{
    Builder, Component, Entities, Entity, LazyUpdate, NullStorage, Read, ReadStorage, ReaderId,
    System, SystemData, World,
};

use feather_core::packet::SpawnObject;
use feather_core::{Item, Packet, Position};

use crate::entity::component::{PacketCreatorComponent, SerializerComponent};
use crate::entity::metadata::Metadata;
use crate::entity::movement::degrees_to_stops;
use crate::entity::{NamedComponent, PositionComponent, VelocityComponent};
use crate::lazy::LazyUpdateExt;
use crate::physics::PhysicsBuilder;
use crate::player::PLAYER_EYE_HEIGHT;
use crate::util::{protocol_velocity, Util};
use feather_core::entity::{ArrowEntityData, BaseEntityData, EntityData};
use specs::world::LazyBuilder;
use uuid::Uuid;

/// Component for arrow entities.
#[derive(Default)]
pub struct ArrowComponent;

impl Component for ArrowComponent {
    type Storage = NullStorage<Self>;
}

/// Event triggered when arrow is shot.
#[derive(Debug, Clone)]
pub struct ShootArrowEvent {
    pub arrow_type: Item,
    pub shooter: Option<Entity>,
    pub position: Position,
    pub critical: bool,
}

#[derive(Default)]
pub struct ShootArrowSystem {
    reader: Option<ReaderId<ShootArrowEvent>>,
}

impl<'a> System<'a> for ShootArrowSystem {
    type SystemData = (
        Read<'a, LazyUpdate>,
        Read<'a, EventChannel<ShootArrowEvent>>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (lazy, shoot_arrow_events, entities) = data;

        for event in shoot_arrow_events.read(self.reader.as_mut().unwrap()) {
            let mut pos = event.position
                + glm::vec3(0.0, PLAYER_EYE_HEIGHT, 0.0)
                + event.position.direction() * 1.5;
            pos.on_ground = false;

            // TODO: Scale velocity based on power
            let velocity = pos.direction();

            // TODO: shooter

            create(lazy.spawn_entity(&entities), false)
                .with(PositionComponent {
                    current: pos,
                    previous: pos,
                })
                .with(VelocityComponent(velocity))
                .build();
        }
    }

    setup_impl!(reader);
}

pub fn create(builder: LazyBuilder, critical: bool) -> LazyBuilder {
    let meta = {
        let mut meta_arrow = crate::entity::metadata::Arrow::default();
        let mask = if critical {
            crate::entity::metadata::ArrowBitMask::CRITICAL
        } else {
            crate::entity::metadata::ArrowBitMask::default()
        };
        meta_arrow.set_arrow_bit_mask(mask.bits());
        // meta_arrow.set_shooter(shooter); TODO
        Metadata::Arrow(meta_arrow)
    };

    builder
        .with(ArrowComponent)
        .with(
            PhysicsBuilder::new()
                .bbox(0.5, 0.5, 0.5)
                .gravity(-0.05)
                .drag(0.99)
                .slip_multiplier(0.0)
                .build(),
        )
        .with(meta)
        .with(PacketCreatorComponent(&create_packet))
        .with(SerializerComponent(&serialize))
}

fn create_packet(world: &World, entity: Entity) -> Box<dyn Packet> {
    let position = world.get::<PositionComponent>().current;
    let (velocity_x, velocity_y, velocity_z) =
        protocol_velocity(world.get::<VelocityComponent>().0);

    let packet = SpawnObject {
        entity_id: entity.id() as i32,
        object_uuid: Uuid::new_v4(), // TODO
        ty: 60,
        x: position.x,
        y: position.y,
        z: position.z,
        pitch: degrees_to_stops(position.pitch),
        yaw: degrees_to_stops(position.yaw),
        data: 1, // TODO: Shooter entity ID
        velocity_x,
        velocity_y,
        velocity_z,
    };

    Box::new(packet)
}

fn serialize(world: &World, entity: Entity) -> EntityData {
    EntityData::Arrow(ArrowEntityData {
        entity: BaseEntityData::new(
            world.get::<PositionComponent>(entity).current,
            world.get::<VelocityComponent>().0,
        ),
        critical: 0, // TODO
    })
}
