use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all trader llamas:
/// ```no_run
/// use quill::{Game, Position, entities::TraderLlamaMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &TraderLlamaMarker)>() {
///         println!("Found a trader llama with position "trader llama"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct TraderLlamaMarker;

pod_component_impl!(TraderLlamaMarker);

/// Entity wrapper for trader llama entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct TraderLlama {
    id: EntityId,
}

wrapper_from_query_impl!(TraderLlama, TraderLlamaMarker);
entity_wrapper_impl!(TraderLlama, TraderLlamaMarker);

impl crate::HasEntityId for TraderLlama {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
