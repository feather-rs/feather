use crate::entity::{
    base_data, create_mob_packet, PacketCreatorComponent, PositionComponent, SerializerComponent,
    VelocityComponent,
};
use crate::lazy::LazyUpdateExt;
use crate::physics::PhysicsBuilder;
use feather_core::entity::{AnimalData, EntityData};
use feather_core::Packet;
use specs::world::{EntitiesRes, LazyBuilder};
use specs::{Builder, Component, Entity, LazyUpdate, NullStorage, World};

#[derive(Default)]
pub struct MooshroomComponent;

impl Component for MooshroomComponent {
    type Storage = NullStorage<Self>;
}

pub fn create<'a>(lazy: &'a LazyUpdate, entities: &'a EntitiesRes) -> LazyBuilder<'a> {
    lazy.spawn_entity(entities)
        .with(MooshroomComponent)
        .with(PhysicsBuilder::for_living().bbox(0.9, 1.4, 0.9).build())
        .with(PacketCreatorComponent(&create_packet))
        .with(SerializerComponent(&serialize))
}

fn create_packet(world: &World, entity: Entity) -> Box<dyn Packet> {
    create_mob_packet(world, entity, 47)
}

pub fn create_from_data(
    lazy: &LazyUpdate,
    entities: &EntitiesRes,
    data: &AnimalData,
) -> Option<Entity> {
    let position = data.base.read_position()?;
    let velocity = data.base.read_velocity()?;

    Some(
        create(lazy, entities)
            .with(PositionComponent {
                current: position,
                previous: position,
            })
            .with(VelocityComponent(velocity))
            .build(),
    )
}

fn serialize(world: &World, entity: Entity) -> EntityData {
    let base = base_data(world, entity);
    EntityData::Mooshroom(AnimalData { base })
}
