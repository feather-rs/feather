use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all llama spits:
/// ```no_run
/// use quill::{Game, Position, entities::LlamaSpitMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &LlamaSpitMarker)>() {
///         println!("Found a llama spit with position "llama spit"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct LlamaSpitMarker;

pod_component_impl!(LlamaSpitMarker);

/// Entity wrapper for llama spit entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct LlamaSpit {
    id: EntityId,
}

wrapper_from_query_impl!(LlamaSpit, LlamaSpitMarker);
entity_wrapper_impl!(LlamaSpit, LlamaSpitMarker);

impl crate::HasEntityId for LlamaSpit {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
