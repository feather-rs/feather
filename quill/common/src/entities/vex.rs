use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all vexs:
/// ```no_run
/// use quill::{Game, Position, entities::VexMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &VexMarker)>() {
///         println!("Found a vex with position "vex"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct VexMarker;

pod_component_impl!(VexMarker);

/// Entity wrapper for vex entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Vex {
    id: EntityId,
}

wrapper_from_query_impl!(Vex, VexMarker);
entity_wrapper_impl!(Vex, VexMarker);

impl crate::HasEntityId for Vex {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
