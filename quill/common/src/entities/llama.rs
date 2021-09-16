use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all llamas:
/// ```no_run
/// use quill::{Game, Position, entities::LlamaMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &LlamaMarker)>() {
///         println!("Found a llama with position "llama"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct LlamaMarker;

pod_component_impl!(LlamaMarker);

/// Entity wrapper for llama entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Llama {
    id: EntityId,
}

wrapper_from_query_impl!(Llama, LlamaMarker);
entity_wrapper_impl!(Llama, LlamaMarker);

impl crate::HasEntityId for Llama {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
