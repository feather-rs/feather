use bytemuck::{Pod, Zeroable};
/// Marker component for shulker entities.
///
/// # Example
/// A system that queries for all shulkers:
/// ```no_run
/// use quill::{Game, Position, entities::Shulker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Shulker)>() {
///         println!("Found a shulker with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Shulker;

pod_component_impl!(Shulker);
