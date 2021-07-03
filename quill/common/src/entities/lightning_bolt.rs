use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all lightning bolts:
/// ```no_run
/// use quill::{Game, Position, entities::LightningBoltMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &LightningBoltMarker)>() {
///         println!("Found a lightning bolt with position "lightning bolt"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct LightningBoltMarker;

pod_component_impl!(LightningBoltMarker);

/// Entity wrapper for lightning bolt entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct LightningBolt {
    id: EntityId,
}

wrapper_from_query_impl!(LightningBolt, LightningBoltMarker);
entity_wrapper_impl!(LightningBolt, LightningBoltMarker);

impl crate::HasEntityId for LightningBolt {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
