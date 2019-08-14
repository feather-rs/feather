//! Assorted components relating to physics
//! and systems to initialize them.

use crate::entity::{EntitySpawnEvent, EntityType};
use glm::DVec3;
use ncollide3d::bounding_volume::AABB;
use shrev::EventChannel;
use specs::{Component, DenseVecStorage, Entity, Read, ReaderId, System, WriteStorage};

/// An entity's bounding box.
#[derive(Debug, Clone, Deref, DerefMut)]
pub struct BoundingBoxComponent(AABB<f64>);

impl Component for BoundingBoxComponent {
    type Storage = DenseVecStorage<Self>;
}

impl BoundingBoxComponent {
    /// Returns the difference between the two
    /// corners of this bounding box.
    pub fn size(&self) -> DVec3 {
        self.0.maxs() - self.0.mins()
    }
}

/// Event which is triggered when an entity's
/// velocity changes.
#[derive(Debug, Clone)]
pub struct EntityVelocityUpdateEvent {
    pub entity: Entity,
}

/// System for initializing new entities'
/// physics components.
#[derive(Default)]
pub struct PhysicsInitSystem {
    reader: Option<ReaderId<EntitySpawnEvent>>,
}

impl<'a> System<'a> for PhysicsInitSystem {
    type SystemData = (
        WriteStorage<'a, BoundingBoxComponent>,
        Read<'a, EventChannel<EntitySpawnEvent>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut bboxes, events) = data;

        for event in events.read(self.reader.as_mut().unwrap()) {
            let bbox = bbox_for_type(event.ty);
            if let Some(bbox) = bbox {
                bboxes
                    .insert(event.entity, BoundingBoxComponent(bbox))
                    .unwrap();
            }
        }
    }

    setup_impl!(reader);
}

/// Returns the bounding box for the given entity type.
fn bbox_for_type(ty: EntityType) -> Option<AABB<f64>> {
    match ty {
        EntityType::Item => Some(bbox(0.25, 0.25)),
        EntityType::Player => Some(bbox(0.6, 1.7)),
        _ => None,
    }
}

/// Returns a bounding box with the given width and height.
fn bbox(size_xz: f64, size_y: f64) -> AABB<f64> {
    AABB::new(
        glm::vec3(0.0, 0.0, 0.0).into(),
        glm::vec3(size_xz, size_y, size_xz).into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testframework as t;
    use specs::WorldExt;

    #[test]
    fn test_physics_init() {
        let (mut w, mut d) = t::init_world();

        t::populate_with_air(&mut w); // Prevent entity from getting removed

        let entity = t::add_entity(&mut w, EntityType::Player);

        let event = EntitySpawnEvent {
            entity,
            ty: EntityType::Player,
        };
        t::trigger_event(&w, event);

        d.dispatch(&w);
        w.maintain();

        let bbox = w
            .read_component::<BoundingBoxComponent>()
            .get(entity)
            .unwrap()
            .clone();
        assert_eq!(bbox.0, bbox_for_type(EntityType::Player).unwrap());
    }
}
