use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all magma cubes:
/// ```no_run
/// use quill::{Game, Position, entities::MagmaCubeMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &MagmaCubeMarker)>() {
///         println!("Found a magma cube with position "magma cube"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct MagmaCubeMarker;

pod_component_impl!(MagmaCubeMarker);

/// Entity wrapper for magma cube entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct MagmaCube {
    id: EntityId,
}

wrapper_from_query_impl!(MagmaCube, MagmaCubeMarker);
entity_wrapper_impl!(MagmaCube, MagmaCubeMarker);

impl crate::HasEntityId for MagmaCube {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
