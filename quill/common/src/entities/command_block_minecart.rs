use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all command block minecarts:
/// ```no_run
/// use quill::{Game, Position, entities::CommandBlockMinecartMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &CommandBlockMinecartMarker)>() {
///         println!("Found a command block minecart with position "command block minecart"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct CommandBlockMinecartMarker;

pod_component_impl!(CommandBlockMinecartMarker);

/// Entity wrapper for command block minecart entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct CommandBlockMinecart {
    id: EntityId,
}

wrapper_from_query_impl!(CommandBlockMinecart, CommandBlockMinecartMarker);
entity_wrapper_impl!(CommandBlockMinecart, CommandBlockMinecartMarker);

impl crate::HasEntityId for CommandBlockMinecart {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
