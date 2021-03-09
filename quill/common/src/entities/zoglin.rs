use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all zoglins:
/// ```no_run
/// use quill::{Game, Position, entities::ZoglinMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &ZoglinMarker)>() {
///         println!("Found a zoglin with position "zoglin"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct ZoglinMarker;

pod_component_impl!(ZoglinMarker);

/// Entity wrapper for zoglin entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Zoglin {
    id: EntityId,
}

wrapper_from_query_impl!(Zoglin, ZoglinMarker);
entity_wrapper_impl!(Zoglin, ZoglinMarker);

impl crate::HasEntityId for Zoglin {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
