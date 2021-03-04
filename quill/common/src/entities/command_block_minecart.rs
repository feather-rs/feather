use bytemuck::{Pod, Zeroable};
/// Marker component for command block minecart entities.
///
/// # Example
/// A system that queries for all command block minecarts:
/// ```no_run
/// use quill::{Game, Position, entities::CommandBlockMinecart};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &CommandBlockMinecart)>() {
///         println!("Found a command block minecart with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct CommandBlockMinecart;

pod_component_impl!(CommandBlockMinecart);
