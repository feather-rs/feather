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
pub struct PigComponent;

impl Component for PigComponent {
    type Storage = NullStorage<Self>;
}

pub fn create<'a>(lazy: &'a LazyUpdate, entities: &'a EntitiesRes) -> LazyBuilder<'a> {
    lazy.spawn_entity(entities)
        .with(PigComponent)
        .with(PhysicsBuilder::for_living().bbox(0.9, 0.9, 0.9).build())
        .with(PacketCreatorComponent(&create_packet))
        .with(SerializerComponent(&serialize))
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

fn create_packet(world: &World, entity: Entity) -> Box<dyn Packet> {
    create_mob_packet(world, entity, 51)
}

fn serialize(world: &World, entity: Entity) -> EntityData {
    let base = base_data(world, entity);
    EntityData::Pig(AnimalData { base })
}
