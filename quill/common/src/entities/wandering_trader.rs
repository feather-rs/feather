use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all wandering traders:
/// ```no_run
/// use quill::{Game, Position, entities::WanderingTraderMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &WanderingTraderMarker)>() {
///         println!("Found a wandering trader with position "wandering trader"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct WanderingTraderMarker;

pod_component_impl!(WanderingTraderMarker);

/// Entity wrapper for wandering trader entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct WanderingTrader {
    id: EntityId,
}

wrapper_from_query_impl!(WanderingTrader, WanderingTraderMarker);
entity_wrapper_impl!(WanderingTrader, WanderingTraderMarker);

impl crate::HasEntityId for WanderingTrader {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
