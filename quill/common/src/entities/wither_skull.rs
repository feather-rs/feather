use bytemuck::{Pod, Zeroable};
/// Marker component for wither skull entities.
///
/// # Example
/// A system that queries for all wither skulls:
/// ```no_run
/// use quill::{Game, Position, entities::WitherSkull};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &WitherSkull)>() {
///         println!("Found a wither skull with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct WitherSkull;

pod_component_impl!(WitherSkull);
