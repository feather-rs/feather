use bytemuck::{Pod, Zeroable};
/// Marker component for chest minecart entities.
///
/// # Example
/// A system that queries for all chest minecarts:
/// ```no_run
/// use quill::{Game, Position, entities::ChestMinecart};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &ChestMinecart)>() {
///         println!("Found a chest minecart with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct ChestMinecart;

pod_component_impl!(ChestMinecart);
