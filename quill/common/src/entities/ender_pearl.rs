use bytemuck::{Pod, Zeroable};
/// Marker component for ender pearl entities.
///
/// # Example
/// A system that queries for all ender pearls:
/// ```no_run
/// use quill::{Game, Position, entities::EnderPearl};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &EnderPearl)>() {
///         println!("Found a ender pearl with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct EnderPearl;

pod_component_impl!(EnderPearl);
