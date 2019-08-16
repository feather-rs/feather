//! Module for performing entity physics, including velocity, drag
//! and position updates each tick.

use rayon::prelude::*;
use shrev::EventChannel;
use specs::{Entities, ParJoin, Read, ReadStorage, System, Write, WriteStorage};

use feather_core::world::block::Block;
use feather_core::world::{ChunkMap, Position};

use crate::entity::{EntityType, PositionComponent, VelocityComponent};
use crate::physics::{block_impacted_by_ray, BoundingBoxComponent};

/// System for updating all entities' positions and velocities
/// each tick.
pub struct EntityPhysicsSystem;

impl<'a> System<'a> for EntityPhysicsSystem {
    type SystemData = (
        WriteStorage<'a, PositionComponent>,
        WriteStorage<'a, VelocityComponent>,
        ReadStorage<'a, BoundingBoxComponent>,
        ReadStorage<'a, EntityType>,
        Read<'a, ChunkMap>,
        Entities<'a>,
    );

    fn run(&mut self, _data: Self::SystemData) {
        // TODO reimplement this
    }
}

/*
/// Retrieves the gravitational acceleration in blocks per tick squared
/// for a given entity type.
///
/// This information was fetched from
/// [the Minecraft wiki](https://minecraft.gamepedia.com/Entity#Motion_of_entities).
fn gravitational_acceleration(ty: EntityType) -> f32 {
    if ty.is_living() {
        -0.08
    } else if ty.is_item() {
        -0.04
    } else {
        0.0
    }
}


/// Retrieves the terminal velocity in blocks per tick
/// for a given entity type.
fn terminal_velocity(ty: EntityType) -> f32 {
    if ty.is_living() {
        -3.92
    } else if ty.is_item() {
        -1.96
    } else {
        0.0
    }
}


/// Retrieves the drag force for a given entity type.
fn drag_force(ty: EntityType) -> f32 {
    if ty.is_living() || ty.is_item() {
        0.98
    } else {
        0.0
    }
}

*/

/*#[allow(clippy::float_cmp)]
#[cfg(test)]
mod tests {
    use crate::entity::{EntityMoveEvent, EntityType};
    use crate::testframework as t;
    use feather_core::world::block::Block;
    use feather_core::world::Position;
    use shrev::ReaderId;
    use specs::{Dispatcher, Entity, World, WorldExt};

    #[test]
    fn test_entity_physics_basic() {
        let (mut w, mut d, entity, mut reader) = setup();

        d.dispatch(&w);
        w.maintain();

        t::assert_not_removed(&w, entity);

        let pos = t::entity_pos(&w, entity);
        assert_eq!(pos.x, 0.0);
        assert_eq!(pos.y, 1.0);
        assert_eq!(pos.z, 0.0);

        let vel = t::entity_vel(&w, entity).unwrap();
        assert_eq!(vel.x, 0.0);
        assert_eq!(vel.z, 0.0);

        let events = t::triggered_events::<EntityMoveEvent>(&w, &mut reader);
        assert_eq!(events.len(), 1);

        let first = events.first().unwrap();
        assert_eq!(first.entity, entity);
        assert_eq!(first.old_pos, position!(0.0, 0.0, 0.0));
        assert_eq!(first.new_pos, position!(0.0, 1.0, 0.0, false));
    }

    #[test]
    fn test_entity_physics_block_collide() {
        let (mut w, mut d, entity, mut reader) = setup();

        t::set_block(0, 1, 0, Block::Stone, &w);

        d.dispatch(&w);
        w.maintain();

        t::assert_not_removed(&w, entity);

        let pos = t::entity_pos(&w, entity);
        assert_eq!(pos, position!(0.0, 0.0, 0.0));

        let vel = t::entity_vel(&w, entity).unwrap();
        assert_eq!(vel, glm::vec3(0.0, 0.0, 0.0));

        let events = t::triggered_events(&w, &mut reader);
        assert!(events.is_empty());
    }

    #[test]
    fn test_entity_physics_unloaded_chunk() {
        let (mut w, mut d, entity, mut reader) = setup();

        t::set_entity_velocity(&w, entity, glm::vec3(10000.0, 0.0, 0.0));

        d.dispatch(&w);
        w.maintain();

        t::assert_removed(&w, entity);

        let events = t::triggered_events(&w, &mut reader);
        assert!(events.is_empty());
    }

    fn setup<'a, 'b>() -> (World, Dispatcher<'a, 'b>, Entity, ReaderId<EntityMoveEvent>) {
        let (mut w, d) = t::init_world();

        let old_pos = position!(0.0, 0.0, 0.0);
        let vel = glm::vec3(0.0, 1.0, 0.0);
        let entity = t::add_entity_with_pos_and_vel(&mut w, EntityType::Player, old_pos, vel, true);

        t::populate_with_air(&mut w);

        let reader = t::reader(&w);

        (w, d, entity, reader)
    }
}
*/
