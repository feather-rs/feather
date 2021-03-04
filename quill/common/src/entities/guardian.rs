use bytemuck::{Pod, Zeroable};
/// Marker component for guardian entities.
///
/// # Example
/// A system that queries for all guardians:
/// ```no_run
/// use quill::{Game, Position, entities::Guardian};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Guardian)>() {
///         println!("Found a guardian with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Guardian;

pod_component_impl!(Guardian);
