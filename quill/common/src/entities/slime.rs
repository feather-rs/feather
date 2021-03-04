use bytemuck::{Pod, Zeroable};
/// Marker component for slime entities.
///
/// # Example
/// A system that queries for all slimes:
/// ```no_run
/// use quill::{Game, Position, entities::Slime};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Slime)>() {
///         println!("Found a slime with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Slime;

pod_component_impl!(Slime);
