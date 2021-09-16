use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all shulker bullets:
/// ```no_run
/// use quill::{Game, Position, entities::ShulkerBulletMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &ShulkerBulletMarker)>() {
///         println!("Found a shulker bullet with position "shulker bullet"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct ShulkerBulletMarker;

pod_component_impl!(ShulkerBulletMarker);

/// Entity wrapper for shulker bullet entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct ShulkerBullet {
    id: EntityId,
}

wrapper_from_query_impl!(ShulkerBullet, ShulkerBulletMarker);
entity_wrapper_impl!(ShulkerBullet, ShulkerBulletMarker);

impl crate::HasEntityId for ShulkerBullet {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
